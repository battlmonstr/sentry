#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutboundMessageData {
    #[prost(enumeration = "MessageId", tag = "1")]
    pub id: i32,
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageByMinBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<OutboundMessageData>,
    #[prost(uint64, tag = "2")]
    pub min_block: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageByIdRequest {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<OutboundMessageData>,
    #[prost(message, optional, tag = "2")]
    pub peer_id: ::core::option::Option<super::types::H512>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageToRandomPeersRequest {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<OutboundMessageData>,
    #[prost(uint64, tag = "2")]
    pub max_peers: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentPeers {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<super::types::H512>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PenalizePeerRequest {
    #[prost(message, optional, tag = "1")]
    pub peer_id: ::core::option::Option<super::types::H512>,
    #[prost(enumeration = "PenaltyKind", tag = "2")]
    pub penalty: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerMinBlockRequest {
    #[prost(message, optional, tag = "1")]
    pub peer_id: ::core::option::Option<super::types::H512>,
    #[prost(uint64, tag = "2")]
    pub min_block: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InboundMessage {
    #[prost(enumeration = "MessageId", tag = "1")]
    pub id: i32,
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
    #[prost(message, optional, tag = "3")]
    pub peer_id: ::core::option::Option<super::types::H512>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forks {
    #[prost(message, optional, tag = "1")]
    pub genesis: ::core::option::Option<super::types::H256>,
    #[prost(uint64, repeated, tag = "2")]
    pub forks: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusData {
    #[prost(uint64, tag = "1")]
    pub network_id: u64,
    #[prost(message, optional, tag = "2")]
    pub total_difficulty: ::core::option::Option<super::types::H256>,
    #[prost(message, optional, tag = "3")]
    pub best_hash: ::core::option::Option<super::types::H256>,
    #[prost(message, optional, tag = "4")]
    pub fork_data: ::core::option::Option<Forks>,
    #[prost(uint64, tag = "5")]
    pub max_block: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageId {
    /// eth64 announcement messages (no id)
    NewBlockHashes = 0,
    NewBlock = 1,
    Transactions = 2,
    /// eth65 announcement messages (no id)
    NewPooledTransactionHashes = 3,
    /// eth66 messages with request-id
    GetBlockHeaders = 4,
    GetBlockBodies = 5,
    GetNodeData = 6,
    GetReceipts = 7,
    GetPooledTransactions = 8,
    BlockHeaders = 9,
    BlockBodies = 10,
    NodeData = 11,
    Receipts = 12,
    PooledTransactions = 13,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PenaltyKind {
    Kick = 0,
}
#[doc = r" Generated client implementations."]
pub mod sentry_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct SentryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SentryClient<tonic::transport::Channel> {
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
    impl<T> SentryClient<T>
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
        pub async fn penalize_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::PenalizePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/PenalizePeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn peer_min_block(
            &mut self,
            request: impl tonic::IntoRequest<super::PeerMinBlockRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/PeerMinBlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message_by_min_block(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMessageByMinBlockRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/SendMessageByMinBlock");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMessageByIdRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/SendMessageById");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message_to_random_peers(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMessageToRandomPeersRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/sentry.Sentry/SendMessageToRandomPeers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn send_message_to_all(
            &mut self,
            request: impl tonic::IntoRequest<super::OutboundMessageData>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/SendMessageToAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusData>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/SetStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn receive_messages(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::InboundMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/ReceiveMessages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn receive_upload_messages(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::InboundMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/ReceiveUploadMessages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn receive_tx_messages(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::InboundMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sentry.Sentry/ReceiveTxMessages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SentryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SentryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SentryClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod sentry_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SentryServer."]
    #[async_trait]
    pub trait Sentry: Send + Sync + 'static {
        async fn penalize_peer(
            &self,
            request: tonic::Request<super::PenalizePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn peer_min_block(
            &self,
            request: tonic::Request<super::PeerMinBlockRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn send_message_by_min_block(
            &self,
            request: tonic::Request<super::SendMessageByMinBlockRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status>;
        async fn send_message_by_id(
            &self,
            request: tonic::Request<super::SendMessageByIdRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status>;
        async fn send_message_to_random_peers(
            &self,
            request: tonic::Request<super::SendMessageToRandomPeersRequest>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status>;
        async fn send_message_to_all(
            &self,
            request: tonic::Request<super::OutboundMessageData>,
        ) -> Result<tonic::Response<super::SentPeers>, tonic::Status>;
        async fn set_status(
            &self,
            request: tonic::Request<super::StatusData>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = "Server streaming response type for the ReceiveMessages method."]
        type ReceiveMessagesStream: Stream<Item = Result<super::InboundMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn receive_messages(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::ReceiveMessagesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReceiveUploadMessages method."]
        type ReceiveUploadMessagesStream: Stream<Item = Result<super::InboundMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn receive_upload_messages(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::ReceiveUploadMessagesStream>, tonic::Status>;
        #[doc = "Server streaming response type for the ReceiveTxMessages method."]
        type ReceiveTxMessagesStream: Stream<Item = Result<super::InboundMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn receive_tx_messages(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<Self::ReceiveTxMessagesStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SentryServer<T: Sentry> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Sentry> SentryServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SentryServer<T>
    where
        T: Sentry,
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
                "/sentry.Sentry/PenalizePeer" => {
                    #[allow(non_camel_case_types)]
                    struct PenalizePeerSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::PenalizePeerRequest> for PenalizePeerSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PenalizePeerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).penalize_peer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PenalizePeerSvc(inner);
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
                "/sentry.Sentry/PeerMinBlock" => {
                    #[allow(non_camel_case_types)]
                    struct PeerMinBlockSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::PeerMinBlockRequest> for PeerMinBlockSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PeerMinBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).peer_min_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PeerMinBlockSvc(inner);
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
                "/sentry.Sentry/SendMessageByMinBlock" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageByMinBlockSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::SendMessageByMinBlockRequest>
                        for SendMessageByMinBlockSvc<T>
                    {
                        type Response = super::SentPeers;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMessageByMinBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).send_message_by_min_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMessageByMinBlockSvc(inner);
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
                "/sentry.Sentry/SendMessageById" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageByIdSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::SendMessageByIdRequest>
                        for SendMessageByIdSvc<T>
                    {
                        type Response = super::SentPeers;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMessageByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_message_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMessageByIdSvc(inner);
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
                "/sentry.Sentry/SendMessageToRandomPeers" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageToRandomPeersSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry>
                        tonic::server::UnaryService<super::SendMessageToRandomPeersRequest>
                        for SendMessageToRandomPeersSvc<T>
                    {
                        type Response = super::SentPeers;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMessageToRandomPeersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).send_message_to_random_peers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMessageToRandomPeersSvc(inner);
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
                "/sentry.Sentry/SendMessageToAll" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageToAllSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::OutboundMessageData> for SendMessageToAllSvc<T> {
                        type Response = super::SentPeers;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OutboundMessageData>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_message_to_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SendMessageToAllSvc(inner);
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
                "/sentry.Sentry/SetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct SetStatusSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::UnaryService<super::StatusData> for SetStatusSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusData>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetStatusSvc(inner);
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
                "/sentry.Sentry/ReceiveMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveMessagesSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::ServerStreamingService<()> for ReceiveMessagesSvc<T> {
                        type Response = super::InboundMessage;
                        type ResponseStream = T::ReceiveMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).receive_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReceiveMessagesSvc(inner);
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
                "/sentry.Sentry/ReceiveUploadMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveUploadMessagesSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::ServerStreamingService<()> for ReceiveUploadMessagesSvc<T> {
                        type Response = super::InboundMessage;
                        type ResponseStream = T::ReceiveUploadMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).receive_upload_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReceiveUploadMessagesSvc(inner);
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
                "/sentry.Sentry/ReceiveTxMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ReceiveTxMessagesSvc<T: Sentry>(pub Arc<T>);
                    impl<T: Sentry> tonic::server::ServerStreamingService<()> for ReceiveTxMessagesSvc<T> {
                        type Response = super::InboundMessage;
                        type ResponseStream = T::ReceiveTxMessagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).receive_tx_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReceiveTxMessagesSvc(inner);
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
    impl<T: Sentry> Clone for SentryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Sentry> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Sentry> tonic::transport::NamedService for SentryServer<T> {
        const NAME: &'static str = "sentry.Sentry";
    }
}
