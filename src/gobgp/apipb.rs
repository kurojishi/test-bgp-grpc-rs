#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartBgpRequest {
    #[prost(message, optional, tag = "1")]
    pub global: ::core::option::Option<Global>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopBgpRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBgpRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBgpResponse {
    #[prost(message, optional, tag = "1")]
    pub global: ::core::option::Option<Global>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchEventRequest {
    #[prost(message, optional, tag = "1")]
    pub peer: ::core::option::Option<watch_event_request::Peer>,
    #[prost(message, optional, tag = "2")]
    pub table: ::core::option::Option<watch_event_request::Table>,
}
/// Nested message and enum types in `WatchEventRequest`.
pub mod watch_event_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Peer {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Table {
        #[prost(message, repeated, tag = "1")]
        pub filters: ::prost::alloc::vec::Vec<table::Filter>,
    }
    /// Nested message and enum types in `Table`.
    pub mod table {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Filter {
            #[prost(enumeration = "filter::Type", tag = "1")]
            pub r#type: i32,
            #[prost(bool, tag = "2")]
            pub init: bool,
        }
        /// Nested message and enum types in `Filter`.
        pub mod filter {
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum Type {
                Best = 0,
                Adjin = 1,
                PostPolicy = 2,
            }
            impl Type {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Type::Best => "BEST",
                        Type::Adjin => "ADJIN",
                        Type::PostPolicy => "POST_POLICY",
                    }
                }
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchEventResponse {
    #[prost(oneof = "watch_event_response::Event", tags = "2, 3")]
    pub event: ::core::option::Option<watch_event_response::Event>,
}
/// Nested message and enum types in `WatchEventResponse`.
pub mod watch_event_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerEvent {
        #[prost(enumeration = "peer_event::Type", tag = "1")]
        pub r#type: i32,
        #[prost(message, optional, tag = "2")]
        pub peer: ::core::option::Option<super::Peer>,
    }
    /// Nested message and enum types in `PeerEvent`.
    pub mod peer_event {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Type {
            Unknown = 0,
            Init = 1,
            EndOfInit = 2,
            State = 3,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unknown => "UNKNOWN",
                    Type::Init => "INIT",
                    Type::EndOfInit => "END_OF_INIT",
                    Type::State => "STATE",
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableEvent {
        #[prost(message, repeated, tag = "2")]
        pub paths: ::prost::alloc::vec::Vec<super::Path>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "2")]
        Peer(PeerEvent),
        #[prost(message, tag = "3")]
        Table(TableEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerRequest {
    #[prost(message, optional, tag = "1")]
    pub peer: ::core::option::Option<Peer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub interface: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enable_advertised: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeerResponse {
    #[prost(message, optional, tag = "1")]
    pub peer: ::core::option::Option<Peer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePeerRequest {
    #[prost(message, optional, tag = "1")]
    pub peer: ::core::option::Option<Peer>,
    /// Calls SoftResetIn after updating the peer configuration if needed.
    #[prost(bool, tag = "2")]
    pub do_soft_reset_in: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePeerResponse {
    /// Indicates whether calling SoftResetIn is required due to this update. If
    /// "true" is set, the client should call SoftResetIn manually. If
    /// "do_soft_reset_in = true" is set in the request, always returned with
    /// "false".
    #[prost(bool, tag = "1")]
    pub needs_soft_reset_in: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub communication: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub soft: bool,
    #[prost(enumeration = "reset_peer_request::SoftResetDirection", tag = "4")]
    pub direction: i32,
}
/// Nested message and enum types in `ResetPeerRequest`.
pub mod reset_peer_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SoftResetDirection {
        In = 0,
        Out = 1,
        Both = 2,
    }
    impl SoftResetDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SoftResetDirection::In => "IN",
                SoftResetDirection::Out => "OUT",
                SoftResetDirection::Both => "BOTH",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownPeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub communication: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnablePeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisablePeerRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub communication: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPeerGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub peer_group: ::core::option::Option<PeerGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePeerGroupRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePeerGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub peer_group: ::core::option::Option<PeerGroup>,
    #[prost(bool, tag = "2")]
    pub do_soft_reset_in: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePeerGroupResponse {
    #[prost(bool, tag = "1")]
    pub needs_soft_reset_in: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeerGroupRequest {
    #[prost(string, tag = "1")]
    pub peer_group_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPeerGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub peer_group: ::core::option::Option<PeerGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDynamicNeighborRequest {
    #[prost(message, optional, tag = "1")]
    pub dynamic_neighbor: ::core::option::Option<DynamicNeighbor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDynamicNeighborRequest {
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub peer_group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDynamicNeighborRequest {
    #[prost(string, tag = "1")]
    pub peer_group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDynamicNeighborResponse {
    #[prost(message, optional, tag = "1")]
    pub dynamic_neighbor: ::core::option::Option<DynamicNeighbor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPathRequest {
    #[prost(enumeration = "TableType", tag = "1")]
    pub table_type: i32,
    #[prost(string, tag = "2")]
    pub vrf_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub path: ::core::option::Option<Path>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPathResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePathRequest {
    #[prost(enumeration = "TableType", tag = "1")]
    pub table_type: i32,
    #[prost(string, tag = "2")]
    pub vrf_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub family: ::core::option::Option<Family>,
    #[prost(message, optional, tag = "4")]
    pub path: ::core::option::Option<Path>,
    #[prost(bytes = "vec", tag = "5")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
}
/// API representation of table.LookupPrefix
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableLookupPrefix {
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(enumeration = "table_lookup_prefix::Type", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `TableLookupPrefix`.
pub mod table_lookup_prefix {
    /// API representation of table.LookupOption
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Exact = 0,
        Longer = 1,
        Shorter = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Exact => "EXACT",
                Type::Longer => "LONGER",
                Type::Shorter => "SHORTER",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPathRequest {
    #[prost(enumeration = "TableType", tag = "1")]
    pub table_type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub family: ::core::option::Option<Family>,
    #[prost(message, repeated, tag = "4")]
    pub prefixes: ::prost::alloc::vec::Vec<TableLookupPrefix>,
    #[prost(enumeration = "list_path_request::SortType", tag = "5")]
    pub sort_type: i32,
    #[prost(bool, tag = "6")]
    pub enable_filtered: bool,
    #[prost(bool, tag = "7")]
    pub enable_nlri_binary: bool,
    #[prost(bool, tag = "8")]
    pub enable_attribute_binary: bool,
    /// enable_only_binary == true means that only nlri_binary and pattrs_binary
    /// will be used instead of nlri and pattrs for each Path in ListPathResponse.
    #[prost(bool, tag = "9")]
    pub enable_only_binary: bool,
}
/// Nested message and enum types in `ListPathRequest`.
pub mod list_path_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SortType {
        None = 0,
        Prefix = 1,
    }
    impl SortType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SortType::None => "NONE",
                SortType::Prefix => "PREFIX",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPathResponse {
    #[prost(message, optional, tag = "1")]
    pub destination: ::core::option::Option<Destination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPathStreamRequest {
    #[prost(enumeration = "TableType", tag = "1")]
    pub table_type: i32,
    #[prost(string, tag = "2")]
    pub vrf_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub paths: ::prost::alloc::vec::Vec<Path>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    #[prost(enumeration = "TableType", tag = "1")]
    pub table_type: i32,
    #[prost(message, optional, tag = "2")]
    pub family: ::core::option::Option<Family>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableResponse {
    #[prost(uint64, tag = "1")]
    pub num_destination: u64,
    #[prost(uint64, tag = "2")]
    pub num_path: u64,
    /// only meaningful when type == ADJ_IN
    #[prost(uint64, tag = "3")]
    pub num_accepted: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddVrfRequest {
    #[prost(message, optional, tag = "1")]
    pub vrf: ::core::option::Option<Vrf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVrfRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVrfRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVrfResponse {
    #[prost(message, optional, tag = "1")]
    pub vrf: ::core::option::Option<Vrf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
    /// if this flag is set, gobgpd won't define new statements
    /// but refer existing statements using statement's names in this arguments.
    #[prost(bool, tag = "2")]
    pub refer_existing_statements: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyRequest {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
    /// if this flag is set, gobgpd won't delete any statements
    /// even if some statements get not used by any policy by this operation.
    #[prost(bool, tag = "2")]
    pub preserve_statements: bool,
    #[prost(bool, tag = "3")]
    pub all: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyResponse {
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPoliciesRequest {
    #[prost(message, repeated, tag = "1")]
    pub defined_sets: ::prost::alloc::vec::Vec<DefinedSet>,
    #[prost(message, repeated, tag = "2")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    #[prost(message, repeated, tag = "3")]
    pub assignments: ::prost::alloc::vec::Vec<PolicyAssignment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDefinedSetRequest {
    #[prost(message, optional, tag = "1")]
    pub defined_set: ::core::option::Option<DefinedSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDefinedSetRequest {
    #[prost(message, optional, tag = "1")]
    pub defined_set: ::core::option::Option<DefinedSet>,
    #[prost(bool, tag = "2")]
    pub all: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDefinedSetRequest {
    #[prost(enumeration = "DefinedType", tag = "1")]
    pub defined_type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDefinedSetResponse {
    #[prost(message, optional, tag = "1")]
    pub defined_set: ::core::option::Option<DefinedSet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStatementRequest {
    #[prost(message, optional, tag = "1")]
    pub statement: ::core::option::Option<Statement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStatementRequest {
    #[prost(message, optional, tag = "1")]
    pub statement: ::core::option::Option<Statement>,
    #[prost(bool, tag = "2")]
    pub all: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatementRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStatementResponse {
    #[prost(message, optional, tag = "1")]
    pub statement: ::core::option::Option<Statement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPolicyAssignmentRequest {
    #[prost(message, optional, tag = "1")]
    pub assignment: ::core::option::Option<PolicyAssignment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyAssignmentRequest {
    #[prost(message, optional, tag = "1")]
    pub assignment: ::core::option::Option<PolicyAssignment>,
    #[prost(bool, tag = "2")]
    pub all: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyAssignmentRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "PolicyDirection", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPolicyAssignmentResponse {
    #[prost(message, optional, tag = "1")]
    pub assignment: ::core::option::Option<PolicyAssignment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPolicyAssignmentRequest {
    #[prost(message, optional, tag = "1")]
    pub assignment: ::core::option::Option<PolicyAssignment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRpkiRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(int64, tag = "3")]
    pub lifetime: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRpkiRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRpkiRequest {
    #[prost(message, optional, tag = "1")]
    pub family: ::core::option::Option<Family>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRpkiResponse {
    #[prost(message, optional, tag = "1")]
    pub server: ::core::option::Option<Rpki>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableRpkiRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableRpkiRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetRpkiRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(bool, tag = "3")]
    pub soft: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRpkiTableRequest {
    #[prost(message, optional, tag = "1")]
    pub family: ::core::option::Option<Family>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRpkiTableResponse {
    #[prost(message, optional, tag = "1")]
    pub roa: ::core::option::Option<Roa>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableZebraRequest {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub route_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub version: u32,
    #[prost(bool, tag = "4")]
    pub nexthop_trigger_enable: bool,
    #[prost(uint32, tag = "5")]
    pub nexthop_trigger_delay: u32,
    #[prost(uint32, tag = "6")]
    pub mpls_label_range_size: u32,
    #[prost(string, tag = "7")]
    pub software_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableMrtRequest {
    #[prost(enumeration = "enable_mrt_request::DumpType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub dump_interval: u64,
    #[prost(uint64, tag = "4")]
    pub rotation_interval: u64,
}
/// Nested message and enum types in `EnableMrtRequest`.
pub mod enable_mrt_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DumpType {
        Updates = 0,
        Table = 1,
    }
    impl DumpType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DumpType::Updates => "UPDATES",
                DumpType::Table => "TABLE",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableMrtRequest {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBmpRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
    #[prost(enumeration = "add_bmp_request::MonitoringPolicy", tag = "3")]
    pub policy: i32,
    #[prost(int32, tag = "4")]
    pub statistics_timeout: i32,
    #[prost(string, tag = "5")]
    pub sys_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub sys_descr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AddBmpRequest`.
pub mod add_bmp_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MonitoringPolicy {
        Pre = 0,
        Post = 1,
        Both = 2,
        Local = 3,
        All = 4,
    }
    impl MonitoringPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MonitoringPolicy::Pre => "PRE",
                MonitoringPolicy::Post => "POST",
                MonitoringPolicy::Both => "BOTH",
                MonitoringPolicy::Local => "LOCAL",
                MonitoringPolicy::All => "ALL",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBmpRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBmpRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBmpResponse {
    #[prost(message, optional, tag = "1")]
    pub station: ::core::option::Option<list_bmp_response::BmpStation>,
}
/// Nested message and enum types in `ListBmpResponse`.
pub mod list_bmp_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BmpStation {
        #[prost(message, optional, tag = "1")]
        pub conf: ::core::option::Option<bmp_station::Conf>,
        #[prost(message, optional, tag = "2")]
        pub state: ::core::option::Option<bmp_station::State>,
    }
    /// Nested message and enum types in `BmpStation`.
    pub mod bmp_station {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Conf {
            #[prost(string, tag = "1")]
            pub address: ::prost::alloc::string::String,
            #[prost(uint32, tag = "2")]
            pub port: u32,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct State {
            #[prost(message, optional, tag = "1")]
            pub uptime: ::core::option::Option<::prost_types::Timestamp>,
            #[prost(message, optional, tag = "2")]
            pub downtime: ::core::option::Option<::prost_types::Timestamp>,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Family {
    #[prost(enumeration = "family::Afi", tag = "1")]
    pub afi: i32,
    #[prost(enumeration = "family::Safi", tag = "2")]
    pub safi: i32,
}
/// Nested message and enum types in `Family`.
pub mod family {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Afi {
        Unknown = 0,
        Ip = 1,
        Ip6 = 2,
        L2vpn = 25,
        Ls = 16388,
        Opaque = 16397,
    }
    impl Afi {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Afi::Unknown => "AFI_UNKNOWN",
                Afi::Ip => "AFI_IP",
                Afi::Ip6 => "AFI_IP6",
                Afi::L2vpn => "AFI_L2VPN",
                Afi::Ls => "AFI_LS",
                Afi::Opaque => "AFI_OPAQUE",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Safi {
        Unknown = 0,
        Unicast = 1,
        Multicast = 2,
        MplsLabel = 4,
        Encapsulation = 7,
        Vpls = 65,
        Evpn = 70,
        Ls = 71,
        SrPolicy = 73,
        Mup = 85,
        MplsVpn = 128,
        MplsVpnMulticast = 129,
        RouteTargetConstraints = 132,
        FlowSpecUnicast = 133,
        FlowSpecVpn = 134,
        KeyValue = 241,
    }
    impl Safi {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Safi::Unknown => "SAFI_UNKNOWN",
                Safi::Unicast => "SAFI_UNICAST",
                Safi::Multicast => "SAFI_MULTICAST",
                Safi::MplsLabel => "SAFI_MPLS_LABEL",
                Safi::Encapsulation => "SAFI_ENCAPSULATION",
                Safi::Vpls => "SAFI_VPLS",
                Safi::Evpn => "SAFI_EVPN",
                Safi::Ls => "SAFI_LS",
                Safi::SrPolicy => "SAFI_SR_POLICY",
                Safi::Mup => "SAFI_MUP",
                Safi::MplsVpn => "SAFI_MPLS_VPN",
                Safi::MplsVpnMulticast => "SAFI_MPLS_VPN_MULTICAST",
                Safi::RouteTargetConstraints => "SAFI_ROUTE_TARGET_CONSTRAINTS",
                Safi::FlowSpecUnicast => "SAFI_FLOW_SPEC_UNICAST",
                Safi::FlowSpecVpn => "SAFI_FLOW_SPEC_VPN",
                Safi::KeyValue => "SAFI_KEY_VALUE",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validation {
    #[prost(enumeration = "validation::State", tag = "1")]
    pub state: i32,
    #[prost(enumeration = "validation::Reason", tag = "2")]
    pub reason: i32,
    #[prost(message, repeated, tag = "3")]
    pub matched: ::prost::alloc::vec::Vec<Roa>,
    #[prost(message, repeated, tag = "4")]
    pub unmatched_asn: ::prost::alloc::vec::Vec<Roa>,
    #[prost(message, repeated, tag = "5")]
    pub unmatched_length: ::prost::alloc::vec::Vec<Roa>,
}
/// Nested message and enum types in `Validation`.
pub mod validation {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        None = 0,
        NotFound = 1,
        Valid = 2,
        Invalid = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::None => "STATE_NONE",
                State::NotFound => "STATE_NOT_FOUND",
                State::Valid => "STATE_VALID",
                State::Invalid => "STATE_INVALID",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Reason {
        None = 0,
        Asn = 1,
        Length = 2,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::None => "REASON_NONE",
                Reason::Asn => "REASON_ASN",
                Reason::Length => "REASON_LENGTH",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    /// One of the following defined in "api/attribute.proto":
    /// - IPAddressPrefix
    /// - LabeledIPAddressPrefix
    /// - EncapsulationNLRI
    /// - EVPNEthernetAutoDiscoveryRoute
    /// - EVPNMACIPAdvertisementRoute
    /// - EVPNInclusiveMulticastEthernetTagRoute
    /// - EVPNEthernetSegmentRoute
    /// - EVPNIPPrefixRoute
    /// - EVPNIPMSIRoute
    /// - LabeledVPNIPAddressPrefix
    /// - RouteTargetMembershipNLRI
    /// - FlowSpecNLRI
    /// - VPNFlowSpecNLRI
    /// - OpaqueNLRI
    /// - LsAddrPrefix
    /// - SRPolicyNLRI
    /// - MUPInterworkSegmentDiscoveryRoute
    /// - MUPDirectSegmentDiscoveryRoute
    /// - MUPType1SessionTransformedRoute
    /// - MUPType2SessionTransformedRoute
    #[prost(message, optional, tag = "1")]
    pub nlri: ::core::option::Option<::prost_types::Any>,
    /// Each attribute must be one of *Attribute defined in
    /// "api/attribute.proto".
    #[prost(message, repeated, tag = "2")]
    pub pattrs: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(message, optional, tag = "3")]
    pub age: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "4")]
    pub best: bool,
    #[prost(bool, tag = "5")]
    pub is_withdraw: bool,
    #[prost(message, optional, tag = "7")]
    pub validation: ::core::option::Option<Validation>,
    #[prost(bool, tag = "8")]
    pub no_implicit_withdraw: bool,
    #[prost(message, optional, tag = "9")]
    pub family: ::core::option::Option<Family>,
    #[prost(uint32, tag = "10")]
    pub source_asn: u32,
    #[prost(string, tag = "11")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "12")]
    pub filtered: bool,
    #[prost(bool, tag = "13")]
    pub stale: bool,
    #[prost(bool, tag = "14")]
    pub is_from_external: bool,
    #[prost(string, tag = "15")]
    pub neighbor_ip: ::prost::alloc::string::String,
    /// only paths installed by AddPath API have this
    #[prost(bytes = "vec", tag = "16")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "17")]
    pub is_nexthop_invalid: bool,
    #[prost(uint32, tag = "18")]
    pub identifier: u32,
    #[prost(uint32, tag = "19")]
    pub local_identifier: u32,
    #[prost(bytes = "vec", tag = "20")]
    pub nlri_binary: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "21")]
    pub pattrs_binary: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub paths: ::prost::alloc::vec::Vec<Path>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(message, optional, tag = "1")]
    pub apply_policy: ::core::option::Option<ApplyPolicy>,
    #[prost(message, optional, tag = "2")]
    pub conf: ::core::option::Option<PeerConf>,
    #[prost(message, optional, tag = "3")]
    pub ebgp_multihop: ::core::option::Option<EbgpMultihop>,
    #[prost(message, optional, tag = "4")]
    pub route_reflector: ::core::option::Option<RouteReflector>,
    #[prost(message, optional, tag = "5")]
    pub state: ::core::option::Option<PeerState>,
    #[prost(message, optional, tag = "6")]
    pub timers: ::core::option::Option<Timers>,
    #[prost(message, optional, tag = "7")]
    pub transport: ::core::option::Option<Transport>,
    #[prost(message, optional, tag = "8")]
    pub route_server: ::core::option::Option<RouteServer>,
    #[prost(message, optional, tag = "9")]
    pub graceful_restart: ::core::option::Option<GracefulRestart>,
    #[prost(message, repeated, tag = "10")]
    pub afi_safis: ::prost::alloc::vec::Vec<AfiSafi>,
    #[prost(message, optional, tag = "11")]
    pub ttl_security: ::core::option::Option<TtlSecurity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerGroup {
    #[prost(message, optional, tag = "1")]
    pub apply_policy: ::core::option::Option<ApplyPolicy>,
    #[prost(message, optional, tag = "2")]
    pub conf: ::core::option::Option<PeerGroupConf>,
    #[prost(message, optional, tag = "3")]
    pub ebgp_multihop: ::core::option::Option<EbgpMultihop>,
    #[prost(message, optional, tag = "4")]
    pub route_reflector: ::core::option::Option<RouteReflector>,
    #[prost(message, optional, tag = "5")]
    pub info: ::core::option::Option<PeerGroupState>,
    #[prost(message, optional, tag = "6")]
    pub timers: ::core::option::Option<Timers>,
    #[prost(message, optional, tag = "7")]
    pub transport: ::core::option::Option<Transport>,
    #[prost(message, optional, tag = "8")]
    pub route_server: ::core::option::Option<RouteServer>,
    #[prost(message, optional, tag = "9")]
    pub graceful_restart: ::core::option::Option<GracefulRestart>,
    #[prost(message, repeated, tag = "10")]
    pub afi_safis: ::prost::alloc::vec::Vec<AfiSafi>,
    #[prost(message, optional, tag = "11")]
    pub ttl_security: ::core::option::Option<TtlSecurity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicNeighbor {
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub peer_group: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPolicy {
    #[prost(message, optional, tag = "1")]
    pub in_policy: ::core::option::Option<PolicyAssignment>,
    #[prost(message, optional, tag = "2")]
    pub export_policy: ::core::option::Option<PolicyAssignment>,
    #[prost(message, optional, tag = "3")]
    pub import_policy: ::core::option::Option<PolicyAssignment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrefixLimit {
    #[prost(message, optional, tag = "1")]
    pub family: ::core::option::Option<Family>,
    #[prost(uint32, tag = "2")]
    pub max_prefixes: u32,
    #[prost(uint32, tag = "3")]
    pub shutdown_threshold_pct: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerConf {
    #[prost(string, tag = "1")]
    pub auth_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub local_asn: u32,
    #[prost(string, tag = "4")]
    pub neighbor_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub peer_asn: u32,
    #[prost(string, tag = "6")]
    pub peer_group: ::prost::alloc::string::String,
    #[prost(enumeration = "PeerType", tag = "7")]
    pub r#type: i32,
    #[prost(enumeration = "RemovePrivate", tag = "8")]
    pub remove_private: i32,
    #[prost(bool, tag = "9")]
    pub route_flap_damping: bool,
    #[prost(uint32, tag = "10")]
    pub send_community: u32,
    #[prost(string, tag = "11")]
    pub neighbor_interface: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub vrf: ::prost::alloc::string::String,
    #[prost(uint32, tag = "13")]
    pub allow_own_asn: u32,
    #[prost(bool, tag = "14")]
    pub replace_peer_asn: bool,
    #[prost(bool, tag = "15")]
    pub admin_down: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerGroupConf {
    #[prost(string, tag = "1")]
    pub auth_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub local_asn: u32,
    #[prost(uint32, tag = "4")]
    pub peer_asn: u32,
    #[prost(string, tag = "5")]
    pub peer_group_name: ::prost::alloc::string::String,
    #[prost(enumeration = "PeerType", tag = "6")]
    pub r#type: i32,
    #[prost(enumeration = "RemovePrivate", tag = "7")]
    pub remove_private: i32,
    #[prost(bool, tag = "8")]
    pub route_flap_damping: bool,
    #[prost(uint32, tag = "9")]
    pub send_community: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerGroupState {
    #[prost(string, tag = "1")]
    pub auth_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub local_asn: u32,
    #[prost(uint32, tag = "4")]
    pub peer_asn: u32,
    #[prost(string, tag = "5")]
    pub peer_group_name: ::prost::alloc::string::String,
    #[prost(enumeration = "PeerType", tag = "6")]
    pub r#type: i32,
    #[prost(enumeration = "RemovePrivate", tag = "7")]
    pub remove_private: i32,
    #[prost(bool, tag = "8")]
    pub route_flap_damping: bool,
    #[prost(uint32, tag = "9")]
    pub send_community: u32,
    #[prost(uint32, tag = "10")]
    pub total_paths: u32,
    #[prost(uint32, tag = "11")]
    pub total_prefixes: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TtlSecurity {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint32, tag = "2")]
    pub ttl_min: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EbgpMultihop {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint32, tag = "2")]
    pub multihop_ttl: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteReflector {
    #[prost(bool, tag = "1")]
    pub route_reflector_client: bool,
    #[prost(string, tag = "2")]
    pub route_reflector_cluster_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerState {
    #[prost(string, tag = "1")]
    pub auth_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub local_asn: u32,
    #[prost(message, optional, tag = "4")]
    pub messages: ::core::option::Option<Messages>,
    #[prost(string, tag = "5")]
    pub neighbor_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub peer_asn: u32,
    #[prost(string, tag = "7")]
    pub peer_group: ::prost::alloc::string::String,
    #[prost(enumeration = "PeerType", tag = "8")]
    pub r#type: i32,
    #[prost(message, optional, tag = "9")]
    pub queues: ::core::option::Option<Queues>,
    #[prost(enumeration = "RemovePrivate", tag = "10")]
    pub remove_private: i32,
    #[prost(bool, tag = "11")]
    pub route_flap_damping: bool,
    #[prost(uint32, tag = "12")]
    pub send_community: u32,
    #[prost(enumeration = "peer_state::SessionState", tag = "13")]
    pub session_state: i32,
    #[prost(enumeration = "peer_state::AdminState", tag = "15")]
    pub admin_state: i32,
    #[prost(uint32, tag = "16")]
    pub out_q: u32,
    #[prost(uint32, tag = "17")]
    pub flops: u32,
    /// Each attribute must be one of *Capability defined in
    /// "api/capability.proto".
    #[prost(message, repeated, tag = "18")]
    pub remote_cap: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(message, repeated, tag = "19")]
    pub local_cap: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(string, tag = "20")]
    pub router_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PeerState`.
pub mod peer_state {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SessionState {
        Unknown = 0,
        Idle = 1,
        Connect = 2,
        Active = 3,
        Opensent = 4,
        Openconfirm = 5,
        Established = 6,
    }
    impl SessionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SessionState::Unknown => "UNKNOWN",
                SessionState::Idle => "IDLE",
                SessionState::Connect => "CONNECT",
                SessionState::Active => "ACTIVE",
                SessionState::Opensent => "OPENSENT",
                SessionState::Openconfirm => "OPENCONFIRM",
                SessionState::Established => "ESTABLISHED",
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AdminState {
        Up = 0,
        Down = 1,
        /// prefix counter over limit
        PfxCt = 2,
    }
    impl AdminState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdminState::Up => "UP",
                AdminState::Down => "DOWN",
                AdminState::PfxCt => "PFX_CT",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Messages {
    #[prost(message, optional, tag = "1")]
    pub received: ::core::option::Option<Message>,
    #[prost(message, optional, tag = "2")]
    pub sent: ::core::option::Option<Message>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(uint64, tag = "1")]
    pub notification: u64,
    #[prost(uint64, tag = "2")]
    pub update: u64,
    #[prost(uint64, tag = "3")]
    pub open: u64,
    #[prost(uint64, tag = "4")]
    pub keepalive: u64,
    #[prost(uint64, tag = "5")]
    pub refresh: u64,
    #[prost(uint64, tag = "6")]
    pub discarded: u64,
    #[prost(uint64, tag = "7")]
    pub total: u64,
    #[prost(uint64, tag = "8")]
    pub withdraw_update: u64,
    #[prost(uint64, tag = "9")]
    pub withdraw_prefix: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queues {
    #[prost(uint32, tag = "1")]
    pub input: u32,
    #[prost(uint32, tag = "2")]
    pub output: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timers {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<TimersConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<TimersState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimersConfig {
    #[prost(uint64, tag = "1")]
    pub connect_retry: u64,
    #[prost(uint64, tag = "2")]
    pub hold_time: u64,
    #[prost(uint64, tag = "3")]
    pub keepalive_interval: u64,
    #[prost(uint64, tag = "4")]
    pub minimum_advertisement_interval: u64,
    #[prost(uint64, tag = "5")]
    pub idle_hold_time_after_reset: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimersState {
    #[prost(uint64, tag = "1")]
    pub connect_retry: u64,
    #[prost(uint64, tag = "2")]
    pub hold_time: u64,
    #[prost(uint64, tag = "3")]
    pub keepalive_interval: u64,
    #[prost(uint64, tag = "4")]
    pub minimum_advertisement_interval: u64,
    #[prost(uint64, tag = "5")]
    pub negotiated_hold_time: u64,
    #[prost(message, optional, tag = "6")]
    pub uptime: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub downtime: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transport {
    #[prost(string, tag = "1")]
    pub local_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub local_port: u32,
    #[prost(bool, tag = "3")]
    pub mtu_discovery: bool,
    #[prost(bool, tag = "4")]
    pub passive_mode: bool,
    #[prost(string, tag = "5")]
    pub remote_address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub remote_port: u32,
    #[prost(uint32, tag = "7")]
    pub tcp_mss: u32,
    #[prost(string, tag = "8")]
    pub bind_interface: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteServer {
    #[prost(bool, tag = "1")]
    pub route_server_client: bool,
    #[prost(bool, tag = "2")]
    pub secondary_route: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GracefulRestart {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint32, tag = "2")]
    pub restart_time: u32,
    #[prost(bool, tag = "3")]
    pub helper_only: bool,
    #[prost(uint32, tag = "4")]
    pub deferral_time: u32,
    #[prost(bool, tag = "5")]
    pub notification_enabled: bool,
    #[prost(bool, tag = "6")]
    pub longlived_enabled: bool,
    #[prost(uint32, tag = "7")]
    pub stale_routes_time: u32,
    #[prost(uint32, tag = "8")]
    pub peer_restart_time: u32,
    #[prost(bool, tag = "9")]
    pub peer_restarting: bool,
    #[prost(bool, tag = "10")]
    pub local_restarting: bool,
    #[prost(string, tag = "11")]
    pub mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MpGracefulRestartConfig {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MpGracefulRestartState {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub received: bool,
    #[prost(bool, tag = "3")]
    pub advertised: bool,
    #[prost(bool, tag = "4")]
    pub end_of_rib_received: bool,
    #[prost(bool, tag = "5")]
    pub end_of_rib_sent: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MpGracefulRestart {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<MpGracefulRestartConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<MpGracefulRestartState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AfiSafiConfig {
    #[prost(message, optional, tag = "1")]
    pub family: ::core::option::Option<Family>,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AfiSafiState {
    #[prost(message, optional, tag = "1")]
    pub family: ::core::option::Option<Family>,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    #[prost(uint64, tag = "3")]
    pub received: u64,
    #[prost(uint64, tag = "4")]
    pub accepted: u64,
    #[prost(uint64, tag = "5")]
    pub advertised: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSelectionOptionsConfig {
    #[prost(bool, tag = "1")]
    pub always_compare_med: bool,
    #[prost(bool, tag = "2")]
    pub ignore_as_path_length: bool,
    #[prost(bool, tag = "3")]
    pub external_compare_router_id: bool,
    #[prost(bool, tag = "4")]
    pub advertise_inactive_routes: bool,
    #[prost(bool, tag = "5")]
    pub enable_aigp: bool,
    #[prost(bool, tag = "6")]
    pub ignore_next_hop_igp_metric: bool,
    #[prost(bool, tag = "7")]
    pub disable_best_path_selection: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSelectionOptionsState {
    #[prost(bool, tag = "1")]
    pub always_compare_med: bool,
    #[prost(bool, tag = "2")]
    pub ignore_as_path_length: bool,
    #[prost(bool, tag = "3")]
    pub external_compare_router_id: bool,
    #[prost(bool, tag = "4")]
    pub advertise_inactive_routes: bool,
    #[prost(bool, tag = "5")]
    pub enable_aigp: bool,
    #[prost(bool, tag = "6")]
    pub ignore_next_hop_igp_metric: bool,
    #[prost(bool, tag = "7")]
    pub disable_best_path_selection: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteSelectionOptions {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<RouteSelectionOptionsConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<RouteSelectionOptionsState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseMultiplePathsConfig {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseMultiplePathsState {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EbgpConfig {
    #[prost(bool, tag = "1")]
    pub allow_multiple_asn: bool,
    #[prost(uint32, tag = "2")]
    pub maximum_paths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EbgpState {
    #[prost(bool, tag = "1")]
    pub allow_multiple_asn: bool,
    #[prost(uint32, tag = "2")]
    pub maximum_paths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ebgp {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<EbgpConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<EbgpState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbgpConfig {
    #[prost(uint32, tag = "1")]
    pub maximum_paths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IbgpState {
    #[prost(uint32, tag = "1")]
    pub maximum_paths: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ibgp {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<IbgpConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<IbgpState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseMultiplePaths {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<UseMultiplePathsConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<UseMultiplePathsState>,
    #[prost(message, optional, tag = "3")]
    pub ebgp: ::core::option::Option<Ebgp>,
    #[prost(message, optional, tag = "4")]
    pub ibgp: ::core::option::Option<Ibgp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetMembershipConfig {
    #[prost(uint32, tag = "1")]
    pub deferral_time: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetMembershipState {
    #[prost(uint32, tag = "1")]
    pub deferral_time: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteTargetMembership {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<RouteTargetMembershipConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<RouteTargetMembershipState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongLivedGracefulRestartConfig {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint32, tag = "2")]
    pub restart_time: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongLivedGracefulRestartState {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub received: bool,
    #[prost(bool, tag = "3")]
    pub advertised: bool,
    #[prost(uint32, tag = "4")]
    pub peer_restart_time: u32,
    #[prost(bool, tag = "5")]
    pub peer_restart_timer_expired: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongLivedGracefulRestart {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<LongLivedGracefulRestartConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<LongLivedGracefulRestartState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AfiSafi {
    #[prost(message, optional, tag = "1")]
    pub mp_graceful_restart: ::core::option::Option<MpGracefulRestart>,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<AfiSafiConfig>,
    #[prost(message, optional, tag = "3")]
    pub state: ::core::option::Option<AfiSafiState>,
    #[prost(message, optional, tag = "4")]
    pub apply_policy: ::core::option::Option<ApplyPolicy>,
    /// TODO:
    /// Support the following structures:
    /// - Ipv4Unicast
    /// - Ipv6Unicast
    /// - Ipv4LabelledUnicast
    /// - Ipv6LabelledUnicast
    /// - L3vpnIpv4Unicast
    /// - L3vpnIpv6Unicast
    /// - L3vpnIpv4Multicast
    /// - L3vpnIpv6Multicast
    /// - L2vpnVpls
    /// - L2vpnEvpn
    #[prost(message, optional, tag = "5")]
    pub route_selection_options: ::core::option::Option<RouteSelectionOptions>,
    #[prost(message, optional, tag = "6")]
    pub use_multiple_paths: ::core::option::Option<UseMultiplePaths>,
    #[prost(message, optional, tag = "7")]
    pub prefix_limits: ::core::option::Option<PrefixLimit>,
    #[prost(message, optional, tag = "8")]
    pub route_target_membership: ::core::option::Option<RouteTargetMembership>,
    #[prost(message, optional, tag = "9")]
    pub long_lived_graceful_restart: ::core::option::Option<LongLivedGracefulRestart>,
    #[prost(message, optional, tag = "10")]
    pub add_paths: ::core::option::Option<AddPaths>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPathsConfig {
    #[prost(bool, tag = "1")]
    pub receive: bool,
    #[prost(uint32, tag = "2")]
    pub send_max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPathsState {
    #[prost(bool, tag = "1")]
    pub receive: bool,
    #[prost(uint32, tag = "2")]
    pub send_max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPaths {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<AddPathsConfig>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<AddPathsState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prefix {
    #[prost(string, tag = "1")]
    pub ip_prefix: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub mask_length_min: u32,
    #[prost(uint32, tag = "3")]
    pub mask_length_max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefinedSet {
    #[prost(enumeration = "DefinedType", tag = "1")]
    pub defined_type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub prefixes: ::prost::alloc::vec::Vec<Prefix>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchSet {
    #[prost(enumeration = "match_set::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MatchSet`.
pub mod match_set {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Any = 0,
        All = 1,
        Invert = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Any => "ANY",
                Type::All => "ALL",
                Type::Invert => "INVERT",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsPathLength {
    #[prost(enumeration = "as_path_length::Type", tag = "1")]
    pub r#type: i32,
    #[prost(uint32, tag = "2")]
    pub length: u32,
}
/// Nested message and enum types in `AsPathLength`.
pub mod as_path_length {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Eq = 0,
        Ge = 1,
        Le = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Eq => "EQ",
                Type::Ge => "GE",
                Type::Le => "LE",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Conditions {
    #[prost(message, optional, tag = "1")]
    pub prefix_set: ::core::option::Option<MatchSet>,
    #[prost(message, optional, tag = "2")]
    pub neighbor_set: ::core::option::Option<MatchSet>,
    #[prost(message, optional, tag = "3")]
    pub as_path_length: ::core::option::Option<AsPathLength>,
    #[prost(message, optional, tag = "4")]
    pub as_path_set: ::core::option::Option<MatchSet>,
    #[prost(message, optional, tag = "5")]
    pub community_set: ::core::option::Option<MatchSet>,
    #[prost(message, optional, tag = "6")]
    pub ext_community_set: ::core::option::Option<MatchSet>,
    #[prost(int32, tag = "7")]
    pub rpki_result: i32,
    #[prost(enumeration = "conditions::RouteType", tag = "8")]
    pub route_type: i32,
    #[prost(message, optional, tag = "9")]
    pub large_community_set: ::core::option::Option<MatchSet>,
    #[prost(string, repeated, tag = "10")]
    pub next_hop_in_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "11")]
    pub afi_safi_in: ::prost::alloc::vec::Vec<Family>,
}
/// Nested message and enum types in `Conditions`.
pub mod conditions {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RouteType {
        None = 0,
        Internal = 1,
        External = 2,
        Local = 3,
    }
    impl RouteType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RouteType::None => "ROUTE_TYPE_NONE",
                RouteType::Internal => "ROUTE_TYPE_INTERNAL",
                RouteType::External => "ROUTE_TYPE_EXTERNAL",
                RouteType::Local => "ROUTE_TYPE_LOCAL",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityAction {
    #[prost(enumeration = "community_action::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, repeated, tag = "2")]
    pub communities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `CommunityAction`.
pub mod community_action {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Add = 0,
        Remove = 1,
        Replace = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Add => "ADD",
                Type::Remove => "REMOVE",
                Type::Replace => "REPLACE",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MedAction {
    #[prost(enumeration = "med_action::Type", tag = "1")]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    pub value: i64,
}
/// Nested message and enum types in `MedAction`.
pub mod med_action {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Mod = 0,
        Replace = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Mod => "MOD",
                Type::Replace => "REPLACE",
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsPrependAction {
    #[prost(uint32, tag = "1")]
    pub asn: u32,
    #[prost(uint32, tag = "2")]
    pub repeat: u32,
    #[prost(bool, tag = "3")]
    pub use_left_most: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NexthopAction {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub self_: bool,
    #[prost(bool, tag = "3")]
    pub unchanged: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalPrefAction {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(enumeration = "RouteAction", tag = "1")]
    pub route_action: i32,
    #[prost(message, optional, tag = "2")]
    pub community: ::core::option::Option<CommunityAction>,
    #[prost(message, optional, tag = "3")]
    pub med: ::core::option::Option<MedAction>,
    #[prost(message, optional, tag = "4")]
    pub as_prepend: ::core::option::Option<AsPrependAction>,
    #[prost(message, optional, tag = "5")]
    pub ext_community: ::core::option::Option<CommunityAction>,
    #[prost(message, optional, tag = "6")]
    pub nexthop: ::core::option::Option<NexthopAction>,
    #[prost(message, optional, tag = "7")]
    pub local_pref: ::core::option::Option<LocalPrefAction>,
    #[prost(message, optional, tag = "8")]
    pub large_community: ::core::option::Option<CommunityAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statement {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub conditions: ::core::option::Option<Conditions>,
    #[prost(message, optional, tag = "3")]
    pub actions: ::core::option::Option<Actions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub statements: ::prost::alloc::vec::Vec<Statement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyAssignment {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "PolicyDirection", tag = "2")]
    pub direction: i32,
    #[prost(message, repeated, tag = "4")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    #[prost(enumeration = "RouteAction", tag = "5")]
    pub default_action: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingPolicy {
    #[prost(message, repeated, tag = "1")]
    pub defined_sets: ::prost::alloc::vec::Vec<DefinedSet>,
    #[prost(message, repeated, tag = "2")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Roa {
    #[prost(uint32, tag = "1")]
    pub asn: u32,
    #[prost(uint32, tag = "2")]
    pub prefixlen: u32,
    #[prost(uint32, tag = "3")]
    pub maxlen: u32,
    #[prost(string, tag = "4")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub conf: ::core::option::Option<RpkiConf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vrf {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Route Distinguisher must be one of
    /// RouteDistinguisherTwoOctetAS,
    /// RouteDistinguisherIPAddressAS,
    /// or RouteDistinguisherFourOctetAS.
    #[prost(message, optional, tag = "2")]
    pub rd: ::core::option::Option<::prost_types::Any>,
    /// List of the Import Route Targets. Each must be one of
    /// TwoOctetAsSpecificExtended,
    /// IPv4AddressSpecificExtended,
    /// or FourOctetAsSpecificExtended.
    #[prost(message, repeated, tag = "3")]
    pub import_rt: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// List of the Export Route Targets. Each must be one of
    /// TwoOctetAsSpecificExtended,
    /// IPv4AddressSpecificExtended,
    /// or FourOctetAsSpecificExtended.
    #[prost(message, repeated, tag = "4")]
    pub export_rt: ::prost::alloc::vec::Vec<::prost_types::Any>,
    #[prost(uint32, tag = "5")]
    pub id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultRouteDistance {
    #[prost(uint32, tag = "1")]
    pub external_route_distance: u32,
    #[prost(uint32, tag = "2")]
    pub internal_route_distance: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Global {
    #[prost(uint32, tag = "1")]
    pub asn: u32,
    #[prost(string, tag = "2")]
    pub router_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub listen_port: i32,
    #[prost(string, repeated, tag = "4")]
    pub listen_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, repeated, tag = "5")]
    pub families: ::prost::alloc::vec::Vec<u32>,
    #[prost(bool, tag = "6")]
    pub use_multiple_paths: bool,
    #[prost(message, optional, tag = "7")]
    pub route_selection_options: ::core::option::Option<RouteSelectionOptionsConfig>,
    #[prost(message, optional, tag = "8")]
    pub default_route_distance: ::core::option::Option<DefaultRouteDistance>,
    #[prost(message, optional, tag = "9")]
    pub confederation: ::core::option::Option<Confederation>,
    #[prost(message, optional, tag = "10")]
    pub graceful_restart: ::core::option::Option<GracefulRestart>,
    #[prost(message, optional, tag = "11")]
    pub apply_policy: ::core::option::Option<ApplyPolicy>,
    #[prost(string, tag = "12")]
    pub bind_to_device: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Confederation {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(uint32, tag = "2")]
    pub identifier: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub member_as_list: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpkiConf {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub remote_port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpkiState {
    #[prost(message, optional, tag = "1")]
    pub uptime: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "2")]
    pub downtime: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bool, tag = "3")]
    pub up: bool,
    #[prost(uint32, tag = "4")]
    pub record_ipv4: u32,
    #[prost(uint32, tag = "5")]
    pub record_ipv6: u32,
    #[prost(uint32, tag = "6")]
    pub prefix_ipv4: u32,
    #[prost(uint32, tag = "7")]
    pub prefix_ipv6: u32,
    #[prost(uint32, tag = "8")]
    pub serial: u32,
    #[prost(int64, tag = "9")]
    pub received_ipv4: i64,
    #[prost(int64, tag = "10")]
    pub received_ipv6: i64,
    #[prost(int64, tag = "11")]
    pub serial_notify: i64,
    #[prost(int64, tag = "12")]
    pub cache_reset: i64,
    #[prost(int64, tag = "13")]
    pub cache_response: i64,
    #[prost(int64, tag = "14")]
    pub end_of_data: i64,
    #[prost(int64, tag = "15")]
    pub error: i64,
    #[prost(int64, tag = "16")]
    pub serial_query: i64,
    #[prost(int64, tag = "17")]
    pub reset_query: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rpki {
    #[prost(message, optional, tag = "1")]
    pub conf: ::core::option::Option<RpkiConf>,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<RpkiState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLogLevelRequest {
    #[prost(enumeration = "set_log_level_request::Level", tag = "1")]
    pub level: i32,
}
/// Nested message and enum types in `SetLogLevelRequest`.
pub mod set_log_level_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Level {
        Panic = 0,
        Fatal = 1,
        Error = 2,
        Warn = 3,
        Info = 4,
        Debug = 5,
        Trace = 6,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::Panic => "PANIC",
                Level::Fatal => "FATAL",
                Level::Error => "ERROR",
                Level::Warn => "WARN",
                Level::Info => "INFO",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableType {
    Global = 0,
    Local = 1,
    AdjIn = 2,
    AdjOut = 3,
    Vrf = 4,
}
impl TableType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TableType::Global => "GLOBAL",
            TableType::Local => "LOCAL",
            TableType::AdjIn => "ADJ_IN",
            TableType::AdjOut => "ADJ_OUT",
            TableType::Vrf => "VRF",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerType {
    Internal = 0,
    External = 1,
}
impl PeerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PeerType::Internal => "INTERNAL",
            PeerType::External => "EXTERNAL",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemovePrivate {
    RemoveNone = 0,
    RemoveAll = 1,
    Replace = 2,
}
impl RemovePrivate {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemovePrivate::RemoveNone => "REMOVE_NONE",
            RemovePrivate::RemoveAll => "REMOVE_ALL",
            RemovePrivate::Replace => "REPLACE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DefinedType {
    Prefix = 0,
    Neighbor = 1,
    Tag = 2,
    AsPath = 3,
    Community = 4,
    ExtCommunity = 5,
    LargeCommunity = 6,
    NextHop = 7,
}
impl DefinedType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DefinedType::Prefix => "PREFIX",
            DefinedType::Neighbor => "NEIGHBOR",
            DefinedType::Tag => "TAG",
            DefinedType::AsPath => "AS_PATH",
            DefinedType::Community => "COMMUNITY",
            DefinedType::ExtCommunity => "EXT_COMMUNITY",
            DefinedType::LargeCommunity => "LARGE_COMMUNITY",
            DefinedType::NextHop => "NEXT_HOP",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RouteAction {
    None = 0,
    Accept = 1,
    Reject = 2,
}
impl RouteAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RouteAction::None => "NONE",
            RouteAction::Accept => "ACCEPT",
            RouteAction::Reject => "REJECT",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolicyDirection {
    Unknown = 0,
    Import = 1,
    Export = 2,
}
impl PolicyDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PolicyDirection::Unknown => "UNKNOWN",
            PolicyDirection::Import => "IMPORT",
            PolicyDirection::Export => "EXPORT",
        }
    }
}
/// Generated client implementations.
pub mod gobgp_api_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GobgpApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GobgpApiClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GobgpApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GobgpApiClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GobgpApiClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn start_bgp(
            &mut self,
            request: impl tonic::IntoRequest<super::StartBgpRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/StartBgp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stop_bgp(
            &mut self,
            request: impl tonic::IntoRequest<super::StopBgpRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/StopBgp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_bgp(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBgpRequest>,
        ) -> Result<tonic::Response<super::GetBgpResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/GetBgp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn watch_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchEventRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::WatchEventResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/WatchEvent",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeletePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPeerRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListPeerResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ListPeer");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn update_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePeerRequest>,
        ) -> Result<tonic::Response<super::UpdatePeerResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/UpdatePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn reset_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetPeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ResetPeer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn shutdown_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::ShutdownPeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ShutdownPeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn enable_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::EnablePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/EnablePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn disable_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DisablePeerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DisablePeer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_peer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPeerGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddPeerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_peer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePeerGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeletePeerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_peer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPeerGroupRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListPeerGroupResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListPeerGroup",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn update_peer_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePeerGroupRequest>,
        ) -> Result<tonic::Response<super::UpdatePeerGroupResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/UpdatePeerGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_dynamic_neighbor(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDynamicNeighborRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddDynamicNeighbor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_dynamic_neighbor(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDynamicNeighborRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListDynamicNeighborResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListDynamicNeighbor",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn delete_dynamic_neighbor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDynamicNeighborRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeleteDynamicNeighbor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_path(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPathRequest>,
        ) -> Result<tonic::Response<super::AddPathResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddPath");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_path(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePathRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeletePath",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_path(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPathRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListPathResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ListPath");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_path_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::AddPathStreamRequest,
            >,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddPathStream",
            );
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn get_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableRequest>,
        ) -> Result<tonic::Response<super::GetTableResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/GetTable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_vrf(
            &mut self,
            request: impl tonic::IntoRequest<super::AddVrfRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddVrf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_vrf(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVrfRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/DeleteVrf");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_vrf(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVrfRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListVrfResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ListVrf");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeletePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListPolicyResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListPolicy",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn set_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPoliciesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/SetPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_defined_set(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDefinedSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddDefinedSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_defined_set(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDefinedSetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeleteDefinedSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_defined_set(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDefinedSetRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListDefinedSetResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListDefinedSet",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::AddStatementRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddStatement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStatementRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeleteStatement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_statement(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStatementRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListStatementResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListStatement",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn add_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPolicyAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/AddPolicyAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeletePolicyAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPolicyAssignmentRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::ListPolicyAssignmentResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListPolicyAssignment",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn set_policy_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPolicyAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/SetPolicyAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::AddRpkiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddRpki");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRpkiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DeleteRpki",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRpkiRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListRpkiResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ListRpki");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn enable_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableRpkiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/EnableRpki",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn disable_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableRpkiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DisableRpki",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn reset_rpki(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetRpkiRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ResetRpki");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_rpki_table(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRpkiTableRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListRpkiTableResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/ListRpkiTable",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn enable_zebra(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableZebraRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/EnableZebra",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn enable_mrt(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableMrtRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/EnableMrt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn disable_mrt(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableMrtRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/DisableMrt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_bmp(
            &mut self,
            request: impl tonic::IntoRequest<super::AddBmpRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/AddBmp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_bmp(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBmpRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/DeleteBmp");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_bmp(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBmpRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ListBmpResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/apipb.GobgpApi/ListBmp");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn set_log_level(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLogLevelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/apipb.GobgpApi/SetLogLevel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
