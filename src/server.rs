use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use virt::domain::Domain;

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

struct TryGetDomainResult {
    domain: Option<Domain>,
    success_response: Option<Result<Response<libvirt_api::SuccessResponse>, Status>>,
}

impl LibvirtAPIService {
    fn new(uri: &str) -> LibvirtAPIService {
        LibvirtAPIService {
            conn: ThreadSafeVirtConn::new(uri),
        }
    }

    fn try_get_domain(&self, uuid: Uuid) -> TryGetDomainResult {
        let domain =
            virt::domain::Domain::lookup_by_uuid_string(&self.conn.lock(), &*uuid.to_string());

        return match domain {
            Ok(x) => TryGetDomainResult {
                domain: Some(x),
                success_response: None,
            },
            Err(e) => TryGetDomainResult {
                domain: None,
                success_response: Some(Ok(Response::new(libvirt_api::SuccessResponse {
                    success: false,
                    error: Some(
                        format!(
                            "failed to look up domain with UUID '{}': {}",
                            uuid, e.message
                        )
                        .to_string(),
                    ),
                }))),
            },
        };
    }

    fn return_success(&self) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        return Ok(Response::new(libvirt_api::SuccessResponse {
            success: true,
            error: None,
        }));
    }

    fn return_failure(
        &self,
        message: String,
    ) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        Ok(Response::new(libvirt_api::SuccessResponse {
            success: false,
            error: Some(message),
        }))
    }
}

#[tonic::async_trait]
impl LibvirtApi for LibvirtAPIService {
    type ListDomainsStream = ReceiverStream<Result<libvirt_api::Domain, Status>>;

    async fn list_domains(
        &self,
        request: Request<libvirt_api::ListDomainsRequest>,
    ) -> Result<Response<Self::ListDomainsStream>, Status> {
        println!("list_domains");
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
    ) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        eprintln!("create_domain");
        let uuid = byte_vec_to_uuid(request.into_inner().uuid).unwrap();

        let domain_r = self.try_get_domain(uuid);
        if domain_r.success_response.is_some() {
            return domain_r.success_response.unwrap();
        }
        let domain = domain_r.domain.unwrap();

        return match domain.create() {
            Ok(0) => self.return_success(),
            Ok(x) => self.return_failure(
                format!("virDomainCreate returned a non-0 response: {}", x).to_string(),
            ),
            Err(e) => self.return_failure(e.message),
        };
    }

    async fn destroy_domain(
        &self,
        request: Request<libvirt_api::DestroyDomainRequest>,
    ) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        eprintln!("destroy_domain");
        let uuid = byte_vec_to_uuid(request.into_inner().uuid).unwrap();

        let domain_r = self.try_get_domain(uuid);
        if domain_r.success_response.is_some() {
            return domain_r.success_response.unwrap();
        }
        let domain = domain_r.domain.unwrap();
        let r = domain.destroy();

        return match r {
            Ok(_) => self.return_success(),
            Err(e) => self.return_failure(e.message),
        };
    }

    type ListUSBDevicesStream = ReceiverStream<Result<libvirt_api::UsbDevice, Status>>;

    async fn list_usb_devices(
        &self,
        _: Request<libvirt_api::ListUsbDevicesRequest>,
    ) -> Result<Response<Self::ListUSBDevicesStream>, Status> {
        eprintln!("list_usb_devices");
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

    async fn attach_device(
        &self,
        request: Request<libvirt_api::AttachDeviceRequest>,
    ) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        eprintln!("attach_device");
        let r = request.into_inner();
        let uuid = byte_vec_to_uuid(r.domain_uuid).unwrap();

        let domain_r = self.try_get_domain(uuid);
        if domain_r.success_response.is_some() {
            return domain_r.success_response.unwrap();
        }
        let domain = domain_r.domain.unwrap();

        let attach = domain.attach_device(&*format!(
            "<hostdev mode='subsystem' type='usb' managed='no'><source><vendor id='0x{}'/><product id='0x{}'/></source></hostdev>", 
            r.vendor_id,
            r.product_id
        ));

        return match attach {
            Ok(_) => self.return_success(),
            Err(e) => self.return_failure(e.message),
        };
    }

    async fn detach_device(
        &self,
        request: Request<libvirt_api::DetachDeviceRequest>,
    ) -> Result<Response<libvirt_api::SuccessResponse>, Status> {
        eprintln!("detach_device");
        let r = request.into_inner();
        let uuid = byte_vec_to_uuid(r.domain_uuid).unwrap();

        let domain_r = self.try_get_domain(uuid);
        if domain_r.success_response.is_some() {
            return domain_r.success_response.unwrap();
        }
        let domain = domain_r.domain.unwrap();

        let attach = domain.detach_device(&*format!(
            "<hostdev mode='subsystem' type='usb' managed='no'><source><vendor id='0x{}'/><product id='0x{}'/></source></hostdev>",
            r.vendor_id,
            r.product_id
        ));

        return match attach {
            Ok(_) => self.return_success(),
            Err(e) => self.return_failure(e.message),
        };
    }

    // async fn ListUSBDevices(&self, request: Request<libvirt_api::ListUSBDevicesRequest>)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = LibvirtAPIService::new("qemu:///system");

    println!("Listening");

    Server::builder()
        .add_service(LibvirtApiServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
