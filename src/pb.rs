#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// \[0\] as type
    #[prost(bytes = "vec", tag = "1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataList {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Data>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataMap {
    #[prost(map = "string, message", tag = "1")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, Data>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(uint32, tag = "1")]
    pub transaction_type: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub gas: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub destination: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "7")]
    pub payload: ::core::option::Option<PayloadInfo>,
    #[prost(bytes = "vec", tag = "8")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "9")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractInfo {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "3")]
    pub method: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub abi: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadInfo {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<DataInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}