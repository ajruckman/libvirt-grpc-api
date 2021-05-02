use std::convert::TryInto;
use std::sync::{Arc, Mutex};

use prost::bytes::Bytes;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;

use libvirt_grpc_api::{byte_vec_to_uuid, enumerate_usb_devices};
use schema::schema::DomainState;

use crate::protoc::libvirt_api;
use crate::protoc::libvirt_api::libvirt_api_server::*;
use crate::thread_safe_virt_conn::ThreadSafeVirtConn;

mod protoc;
mod schema;
mod thread_safe_virt_conn;

pub struct LibvirtAPIService {
    conn: ThreadSafeVirtConn,
}

impl LibvirtAPIService {
    fn new(uri: &str) -> LibvirtAPIService {
        LibvirtAPIService {
            conn: ThreadSafeVirtConn::new(uri),
        }
    }

    fn try_get_domain(&self, uuid: Uuid) -> Result<Domain, Error> {
        return virt::domain::Domain::lookup_by_uuid_string(&self.conn.lock(), &*uuid.to_string());
    }
}

#[tonic::async_trait]
impl LibvirtApi for LibvirtAPIService {
    type ListDomainsStream = ReceiverStream<Result<libvirt_api::Domain, Status>>;

    async fn list_domains(
        &self,
        request: Request<libvirt_api::ListDomainsRequest>,
    ) -> Result<Response<Self::ListDomainsStream>, Status> {
        let flags = request.into_inner().flags as virt::connect::ConnectListAllDomainsFlags;

        let domains: Vec<schema::schema::Domain> = self
            .conn
            .lock()
            .list_all_domains(flags)
            .unwrap()
            .iter()
            .map(|x| {
                let info = x.get_info().unwrap();

                schema::schema::Domain {
                    uuid: Uuid::parse_str(x.get_uuid_string().unwrap().as_str()).unwrap(),
                    id: x.get_id().unwrap_or(0),
                    name: x.get_name().unwrap(),
                    hostname: x.get_hostname(0).ok(),
                    os_type: x.get_os_type().ok(),

                    state: match x.get_state() {
                        Ok(s) => match s.0 {
                            virt::domain::VIR_DOMAIN_NOSTATE => DomainState::NoState,
                            virt::domain::VIR_DOMAIN_RUNNING => DomainState::Running,
                            virt::domain::VIR_DOMAIN_BLOCKED => DomainState::Blocked,
                            virt::domain::VIR_DOMAIN_PAUSED => DomainState::Paused,
                            virt::domain::VIR_DOMAIN_SHUTDOWN => DomainState::ShutDown,
                            virt::domain::VIR_DOMAIN_SHUTOFF => DomainState::ShutOff,
                            virt::domain::VIR_DOMAIN_CRASHED => DomainState::Crashed,
                            virt::domain::VIR_DOMAIN_PMSUSPENDED => DomainState::PMSuspended,
                            _ => panic!("Out of bounds"),
                        },
                        Err(_) => DomainState::Unspecified,
                    },
                    memory: x.get_max_memory().unwrap(),
                    memory_max: x.get_max_memory().unwrap(),
                    virt_cpu_num: info.nr_virt_cpu,
                    virt_cpu_time: info.cpu_time,
                }
            })
            .collect::<Vec<_>>()
            .into();

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for v in domains {
                tx.send(Ok(libvirt_api::Domain {
                    uuid: v.uuid.as_bytes().to_vec(),
                    id: v.id,
                    name: v.name,
                    hostname: v.hostname,
                    os_type: v.os_type,
                    state: match v.state {
                        DomainState::Unspecified => libvirt_api::DomainState::Unspecified as i32,
                        DomainState::NoState => libvirt_api::DomainState::Nostate as i32,
                        DomainState::Running => libvirt_api::DomainState::Running as i32,
                        DomainState::Blocked => libvirt_api::DomainState::Blocked as i32,
                        DomainState::Paused => libvirt_api::DomainState::Paused as i32,
                        DomainState::ShutDown => libvirt_api::DomainState::Shutdown as i32,
                        DomainState::ShutOff => libvirt_api::DomainState::Shutoff as i32,
                        DomainState::Crashed => libvirt_api::DomainState::Crashed as i32,
                        DomainState::PMSuspended => libvirt_api::DomainState::Pmsuspended as i32,
                    },
                    memory: v.memory,
                    memory_max: v.memory_max,
                    virt_cpu_num: v.virt_cpu_num,
                    virt_cpu_time: v.virt_cpu_time,
                }))
                .await
                .unwrap();
            }
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    async fn create_domain(
        &self,
        request: Request<libvirt_api::CreateDomainRequest>,
    ) -> Result<Response<libvirt_api::CreateDomainResponse>, Status> {
        let uuid = byte_vec_to_uuid(request.into_inner().uuid).unwrap();

        let domain = match self.try_get_domain(uuid) {
            Ok(x) => x,
            Err(e) => {
                return Ok(Response::new(libvirt_api::CreateDomainResponse {
                    success: false,
                    error: Some(format!("domain with UUID '{}' not found", uuid).to_string()),
                }))
            }
        };

        return match domain.create() {
            Ok(0) => Ok(Response::new(libvirt_api::CreateDomainResponse {
                success: true,
                error: None,
            })),
            Ok(x) => Ok(Response::new(libvirt_api::CreateDomainResponse {
                success: false,
                error: Some(
                    format!("virDomainCreate returned a non-0 response: {}", x).to_string(),
                ),
            })),
            Err(e) => Ok(Response::new(libvirt_api::CreateDomainResponse {
                success: false,
                error: Some(e.message),
            })),
        };
    }

    async fn destroy_domain(
        &self,
        request: Request<libvirt_api::DestroyDomainRequest>,
    ) -> Result<Response<libvirt_api::DestroyDomainResponse>, Status> {
        let uuid = byte_vec_to_uuid(request.into_inner().uuid).unwrap();

        let domain = match self.try_get_domain(uuid) {
            Ok(x) => x,
            Err(e) => {
                return Ok(Response::new(libvirt_api::DestroyDomainResponse {
                    success: false,
                    error: Some(format!("domain with UUID '{}' not found", uuid).to_string()),
                }))
            }
        };

        return match domain.destroy() {
            Ok(v) => Ok(Response::new(libvirt_api::DestroyDomainResponse {
                success: true,
                error: None,
            })),
            Err(e) => Ok(Response::new(libvirt_api::DestroyDomainResponse {
                success: false,
                error: Some(e.message),
            })),
        };
    }

    type ListUSBDevicesStream = ReceiverStream<Result<libvirt_api::UsbDevice, Status>>;

    async fn list_usb_devices(
        &self,
        request: Request<libvirt_api::ListUsbDevicesRequest>,
    ) -> Result<Response<Self::ListUSBDevicesStream>, Status> {
        let devices = enumerate_usb_devices().unwrap();

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for device in devices {
                tx.send(Ok(libvirt_api::UsbDevice {
                    device: device.device,
                    vendor_id: device.vendor_id,
                    product_id: device.product_id,
                    model: device.model,
                    vendor_name: device.vendor_name,
                    model_name: device.model_name,
                }))
                .await
                .unwrap();
            }
        });

        return Ok(Response::new(ReceiverStream::new(rx)));
    }

    // async fn ListUSBDevices(&self, request: Request<libvirt_api::ListUSBDevicesRequest>)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = LibvirtAPIService::new("qemu:///system");

    Server::builder()
        .add_service(LibvirtApiServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
