mod protoc;
mod schema;

use crate::protoc::libvirt_api;
use crate::protoc::libvirt_api::libvirt_api_server::*;

use libvirt_grpc_api::byte_vec_to_uuid;
use prost::bytes::Bytes;
use schema::schema::DomainState;
use std::convert::TryInto;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use uuid::Uuid;
use virt::connect::Connect;

#[derive(Debug)]
pub struct LibvirtAPIService {}

#[tonic::async_trait]
impl LibvirtApi for LibvirtAPIService {
    type ListDomainsStream = ReceiverStream<Result<libvirt_api::Domain, Status>>;

    async fn list_domains(
        &self,
        request: Request<libvirt_api::ListDomainsRequest>,
    ) -> Result<Response<Self::ListDomainsStream>, Status> {
        let flags = request.into_inner().flags as virt::connect::ConnectListAllDomainsFlags;
        let conn = Connect::open("qemu:///system").unwrap();

        let domains: Vec<schema::schema::Domain> = conn
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
                        Err(_) => DomainState::Undefined,
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
                        DomainState::Undefined => libvirt_api::DomainState::Undefined as i32,
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
        let conn = Connect::open("qemu:///system").unwrap();

        let uuid = byte_vec_to_uuid(request.into_inner().uuid).unwrap();

        let domain =
            virt::domain::Domain::lookup_by_uuid_string(&conn, &*uuid.to_string()).unwrap();

        return match domain.create() {
            Ok(0) => Ok(Response::new(libvirt_api::CreateDomainResponse {
                success: true,
                error: None,
            })),
            Ok(x) => Ok(Response::new(libvirt_api::CreateDomainResponse {
                success: true,
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = LibvirtAPIService {};

    Server::builder()
        .add_service(LibvirtApiServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
