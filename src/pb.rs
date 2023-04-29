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
pub struct Block {
    #[prost(uint64, tag = "1")]
    pub block_index: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub transaction_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub state_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
    #[prost(message, repeated, tag = "7")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionWithData>,
    #[prost(bytes = "vec", repeated, tag = "8")]
    pub states: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
    #[prost(uint64, tag = "4")]
    pub gas: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub destination: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "6")]
    pub payload: ::core::option::Option<PayloadInfo>,
    #[prost(bytes = "vec", tag = "7")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Receipt {
    #[prost(uint64, tag = "1")]
    pub block_index: u64,
    #[prost(uint32, tag = "2")]
    pub transaction_index: u32,
    #[prost(uint32, tag = "3")]
    pub transaction_result: u32,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub states: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(uint32, tag = "1")]
    pub state_type: u32,
    #[prost(uint64, tag = "2")]
    pub block_index: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
    #[prost(bytes = "vec", tag = "5")]
    pub previous: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "6")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountState {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<State>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gas: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub user: ::core::option::Option<DataInfo>,
    #[prost(message, optional, tag = "5")]
    pub code: ::core::option::Option<DataInfo>,
    #[prost(message, optional, tag = "6")]
    pub page: ::core::option::Option<DataInfo>,
    #[prost(message, optional, tag = "7")]
    pub token: ::core::option::Option<DataInfo>,
    #[prost(message, optional, tag = "8")]
    pub data: ::core::option::Option<DataInfo>,
    #[prost(message, optional, tag = "9")]
    pub file: ::core::option::Option<DataInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaItem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub desc: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaInfo {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(uint64, tag = "3")]
    pub count: u64,
    #[prost(int64, tag = "4")]
    pub total: i64,
    #[prost(message, repeated, tag = "5")]
    pub items: ::prost::alloc::vec::Vec<MetaItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenItem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenInfo {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub index: u64,
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<TokenItem>,
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
pub struct TransactionWithData {
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag = "2")]
    pub receipt: ::core::option::Option<Receipt>,
    #[prost(int64, tag = "3")]
    pub date: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageKey {
    #[prost(bytes = "vec", tag = "1")]
    pub message_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
