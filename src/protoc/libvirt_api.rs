#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainsRequest {
    #[prost(uint32, tag = "1")]
    pub flags: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub id: u32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub os_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "DomainState", tag = "6")]
    pub state: i32,
    #[prost(uint64, tag = "7")]
    pub memory: u64,
    #[prost(uint64, tag = "8")]
    pub memory_max: u64,
    #[prost(uint32, tag = "9")]
    pub virt_cpu_num: u32,
    #[prost(uint64, tag = "10")]
    pub virt_cpu_time: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDomainRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDomainResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, optional, tag = "2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyDomainRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyDomainResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, optional, tag = "2")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DomainState {
    Undefined = 0,
    Nostate = 1,
    Running = 2,
    Blocked = 3,
    Paused = 4,
    Shutdown = 5,
    Shutoff = 6,
    Crashed = 7,
    Pmsuspended = 8,
}
#[doc = r" Generated client implementations."]
pub mod libvirt_api_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct LibvirtApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LibvirtApiClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LibvirtApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn list_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDomainsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Domain>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/libvirt_api.LibvirtAPI/ListDomains");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn create_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDomainRequest>,
        ) -> Result<tonic::Response<super::CreateDomainResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/libvirt_api.LibvirtAPI/CreateDomain");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn destroy_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyDomainRequest>,
        ) -> Result<tonic::Response<super::DestroyDomainResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/libvirt_api.LibvirtAPI/DestroyDomain");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LibvirtApiClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LibvirtApiClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LibvirtApiClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod libvirt_api_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LibvirtApiServer."]
    #[async_trait]
    pub trait LibvirtApi: Send + Sync + 'static {
        #[doc = "Server streaming response type for the ListDomains method."]
        type ListDomainsStream: futures_core::Stream<Item = Result<super::Domain, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn list_domains(
            &self,
            request: tonic::Request<super::ListDomainsRequest>,
        ) -> Result<tonic::Response<Self::ListDomainsStream>, tonic::Status>;
        async fn create_domain(
            &self,
            request: tonic::Request<super::CreateDomainRequest>,
        ) -> Result<tonic::Response<super::CreateDomainResponse>, tonic::Status>;
        async fn destroy_domain(
            &self,
            request: tonic::Request<super::DestroyDomainRequest>,
        ) -> Result<tonic::Response<super::DestroyDomainResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LibvirtApiServer<T: LibvirtApi> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: LibvirtApi> LibvirtApiServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for LibvirtApiServer<T>
    where
        T: LibvirtApi,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/libvirt_api.LibvirtAPI/ListDomains" => {
                    #[allow(non_camel_case_types)]
                    struct ListDomainsSvc<T: LibvirtApi>(pub Arc<T>);
                    impl<T: LibvirtApi>
                        tonic::server::ServerStreamingService<super::ListDomainsRequest>
                        for ListDomainsSvc<T>
                    {
                        type Response = super::Domain;
                        type ResponseStream = T::ListDomainsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDomainsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_domains(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ListDomainsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/libvirt_api.LibvirtAPI/CreateDomain" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDomainSvc<T: LibvirtApi>(pub Arc<T>);
                    impl<T: LibvirtApi> tonic::server::UnaryService<super::CreateDomainRequest> for CreateDomainSvc<T> {
                        type Response = super::CreateDomainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDomainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_domain(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/libvirt_api.LibvirtAPI/DestroyDomain" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyDomainSvc<T: LibvirtApi>(pub Arc<T>);
                    impl<T: LibvirtApi> tonic::server::UnaryService<super::DestroyDomainRequest>
                        for DestroyDomainSvc<T>
                    {
                        type Response = super::DestroyDomainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyDomainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).destroy_domain(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DestroyDomainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: LibvirtApi> Clone for LibvirtApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: LibvirtApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: LibvirtApi> tonic::transport::NamedService for LibvirtApiServer<T> {
        const NAME: &'static str = "libvirt_api.LibvirtAPI";
    }
}
