#[derive(Clone, PartialEq, ::prost::Message)]
pub struct H128 {
    #[prost(uint64, tag = "1")]
    pub hi: u64,
    #[prost(uint64, tag = "2")]
    pub lo: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct H160 {
    #[prost(message, optional, tag = "1")]
    pub hi: ::core::option::Option<H128>,
    #[prost(uint32, tag = "2")]
    pub lo: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct H256 {
    #[prost(message, optional, tag = "1")]
    pub hi: ::core::option::Option<H128>,
    #[prost(message, optional, tag = "2")]
    pub lo: ::core::option::Option<H128>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct H512 {
    #[prost(message, optional, tag = "1")]
    pub hi: ::core::option::Option<H256>,
    #[prost(message, optional, tag = "2")]
    pub lo: ::core::option::Option<H256>,
}
/// Reply message containing the current service version on the service side
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReply {
    #[prost(uint32, tag = "1")]
    pub major: u32,
    #[prost(uint32, tag = "2")]
    pub minor: u32,
    #[prost(uint32, tag = "3")]
    pub patch: u32,
}
