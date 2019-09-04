/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfDescribeGroupsRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfDescribeGroupsRequest {
    /// The names of the groups to describe
    pub groups: Vec<String>,
}

// -----------------------------------
// KfDescribeGroupsResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfDescribeGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub throttle_time_ms: i32,

    /// Each described group.
    pub groups: Vec<DescribedGroup>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct DescribedGroup {
    /// The describe error, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The group ID string.
    pub group_id: String,

    /// The group state string, or the empty string.
    pub group_state: String,

    /// The group protocol type, or the empty string.
    pub protocol_type: String,

    /// The group protocol data, or the empty string.
    pub protocol_data: String,

    /// The group members.
    pub members: Vec<DescribedGroupMember>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct DescribedGroupMember {
    /// The member ID assigned by the group coordinator.
    pub member_id: String,

    /// The client ID used in the member's latest join group request.
    pub client_id: String,

    /// The client host.
    pub client_host: String,

    /// The metadata corresponding to the current group protocol in use.
    pub member_metadata: Vec<u8>,

    /// The current assignment provided by the group leader.
    pub member_assignment: Vec<u8>,
}

// -----------------------------------
// Implementation - KfDescribeGroupsRequest
// -----------------------------------

impl Request for KfDescribeGroupsRequest {
    const API_KEY: u16 = 15;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 2;
    const DEFAULT_API_VERSION: i16 = 2;

    type Response = KfDescribeGroupsResponse;
}
