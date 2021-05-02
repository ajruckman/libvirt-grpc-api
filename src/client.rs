use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::result::Result;

use async_trait::async_trait;
use tonic::transport::Channel;
use tonic::Status;
use uuid::Uuid;
use virt::domain::{VIR_DOMAIN_PAUSED, VIR_DOMAIN_RUNNING};

use libvirt_grpc_api::*;
use schema::schema::DomainState;

use crate::protoc::libvirt_api;
use crate::protoc::libvirt_api::libvirt_api_client::*;
use crate::protoc::libvirt_api::CreateDomainRequest;

mod protoc;
mod schema;

#[async_trait]
pub trait LibvirtAPIClient {
    async fn list_domains(
        &mut self,
    ) -> Result<Vec<schema::schema::Domain>, libvirt_grpc_api::GRPCAPIError>;
    async fn create_domain(&mut self, uuid: Uuid) -> Result<(), libvirt_grpc_api::GRPCAPIError>;
    async fn destroy_domain(&mut self, uuid: Uuid) -> Result<(), libvirt_grpc_api::GRPCAPIError>;

    async fn list_usb_devices(
        &mut self,
    ) -> Result<Vec<schema::schema::USBDevice>, libvirt_grpc_api::GRPCAPIError>;
}

pub struct GRPCLibvirtAPIClient {
    client: LibvirtApiClient<Channel>,
}

impl GRPCLibvirtAPIClient {
    pub async fn new(dst: String) -> Result<GRPCLibvirtAPIClient, tonic::transport::Error> {
        let mut client = LibvirtApiClient::connect(dst).await?;

        return Ok(GRPCLibvirtAPIClient { client });
    }
}

#[async_trait]
impl LibvirtAPIClient for GRPCLibvirtAPIClient {
    async fn list_domains(&mut self) -> Result<Vec<schema::schema::Domain>, GRPCAPIError> {
        let request = tonic::Request::new(libvirt_api::ListDomainsRequest {
            flags: VIR_DOMAIN_RUNNING | VIR_DOMAIN_PAUSED,
        });

        let mut stream = self.client.list_domains(request).await?.into_inner();

        let mut res: Vec<schema::schema::Domain> = Vec::new();

        while let Some(domain) = stream.message().await? {
            let uuid = byte_vec_to_uuid(domain.uuid).unwrap();

            res.push(schema::schema::Domain {
                uuid: uuid,
                id: domain.id,
                name: domain.name,
                hostname: domain.hostname,
                os_type: domain.os_type,
                state: match protoc::libvirt_api::DomainState::from_i32(domain.state) {
                    Some(libvirt_api::DomainState::Unspecified) => DomainState::Unspecified,
                    Some(libvirt_api::DomainState::Nostate) => DomainState::NoState,
                    Some(libvirt_api::DomainState::Running) => DomainState::Running,
                    Some(libvirt_api::DomainState::Blocked) => DomainState::Blocked,
                    Some(libvirt_api::DomainState::Paused) => DomainState::Paused,
                    Some(libvirt_api::DomainState::Shutdown) => DomainState::ShutDown,
                    Some(libvirt_api::DomainState::Shutoff) => DomainState::ShutOff,
                    Some(libvirt_api::DomainState::Crashed) => DomainState::Crashed,
                    Some(libvirt_api::DomainState::Pmsuspended) => DomainState::PMSuspended,
                    None => schema::schema::DomainState::Unspecified,
                },
                memory: domain.memory,
                memory_max: domain.memory_max,
                virt_cpu_num: domain.virt_cpu_num,
                virt_cpu_time: domain.virt_cpu_time,
            })
        }

        return Ok(res);
    }

    async fn create_domain(&mut self, uuid: Uuid) -> Result<(), GRPCAPIError> {
        let response = self
            .client
            .create_domain(CreateDomainRequest {
                uuid: uuid.as_bytes().to_vec(),
            })
            .await?;

        let msg = response.into_inner();

        if !msg.success {
            return Err(GRPCAPIError::new(msg.error.unwrap()));
        }

        return Ok(());
    }

    async fn destroy_domain(&mut self, uuid: Uuid) -> Result<(), GRPCAPIError> {
        todo!()
    }

    async fn list_usb_devices(&mut self) -> Result<Vec<schema::schema::USBDevice>, GRPCAPIError> {
        let mut stream = self
            .client
            .list_usb_devices(libvirt_api::ListUsbDevicesRequest {})
            .await?
            .into_inner();

        let mut res: Vec<schema::schema::USBDevice> = Vec::new();

        while let Some(device) = stream.message().await? {
            res.push(schema::schema::USBDevice {
                device: device.device,
                vendor_id: device.vendor_id,
                product_id: device.product_id,
                model: device.model,
                vendor_name: device.vendor_name,
                model_name: device.model_name,
            })
        }

        return Ok(res);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GRPCLibvirtAPIClient::new("http://[::1]:50051".to_string())
        .await
        .unwrap();

    let domains = client.list_domains().await.unwrap();

    for x in &domains {
        println!("{:?}", x);
    }

    let i686 = domains.iter().find(|x| x.name == "vm-i686").unwrap();

    // client.create_domain(Uuid::new_v4()).await.unwrap();

    let devices = client.list_usb_devices().await.unwrap();

    for x in devices {
        println!("{}", x);
    }

    Ok(())

    // let mut client = LibvirtApiClient::connect("http://[::1]:50051").await?;
    //
    // let request = tonic::Request::new(ListDomainsRequest {
    //     flags: VIR_DOMAIN_RUNNING | VIR_DOMAIN_PAUSED,
    // });
    //
    // let mut stream = client.list_domains(request).await?.into_inner();
    //
    // while let Some(domain) = stream.message().await? {
    //     println!("{:?}", domain);
    // }
    //
    // Ok(())
}
