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
// KfMetadataRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfMetadataRequest {
    /// The topics to fetch metadata for.
    pub topics: Option<Vec<MetadataRequestTopic>>,

    /// If this is true, the broker may auto-create topics that we requested which do not already
    /// exist, if it is configured to do so.
    #[fluvio_kf(min_version = 4)]
    pub allow_auto_topic_creation: bool,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct MetadataRequestTopic {
    /// The topic name.
    pub name: String,
}

// -----------------------------------
// KfMetadataResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfMetadataResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation,
    /// or zero if the request did not violate any quota.
    #[fluvio_kf(min_version = 3)]
    pub throttle_time_ms: i32,

    /// Each broker in the response.
    pub brokers: Vec<MetadataResponseBroker>,

    /// The cluster ID that responding broker belongs to.
    #[fluvio_kf(min_version = 2, ignorable)]
    pub cluster_id: Option<String>,

    /// The ID of the controller broker.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub controller_id: i32,

    /// Each topic in the response.
    pub topics: Vec<MetadataResponseTopic>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct MetadataResponseBroker {
    /// The broker ID.
    pub node_id: i32,

    /// The broker hostname.
    pub host: String,

    /// The broker port.
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub rack: Option<String>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct MetadataResponseTopic {
    /// The topic error, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The topic name.
    pub name: String,

    /// True if the topic is internal.
    #[fluvio_kf(min_version = 1, ignorable)]
    pub is_internal: bool,

    /// Each partition in the topic.
    pub partitions: Vec<MetadataResponsePartition>,
}

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct MetadataResponsePartition {
    /// The partition error, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The partition index.
    pub partition_index: i32,

    /// The ID of the leader broker.
    pub leader_id: i32,

    /// The leader epoch of this partition.
    #[fluvio_kf(min_version = 7, ignorable)]
    pub leader_epoch: i32,

    /// The set of all nodes that host this partition.
    pub replica_nodes: Vec<i32>,

    /// The set of nodes that are in sync with the leader for this partition.
    pub isr_nodes: Vec<i32>,

    /// The set of offline replicas of this partition.
    #[fluvio_kf(min_version = 5, ignorable)]
    pub offline_replicas: Vec<i32>,
}

// -----------------------------------
// Implementation - KfMetadataRequest
// -----------------------------------

impl Request for KfMetadataRequest {
    const API_KEY: u16 = 3;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 7;
    const DEFAULT_API_VERSION: i16 = 7;

    type Response = KfMetadataResponse;
}
