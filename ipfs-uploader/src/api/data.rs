use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use crate::api::metadata::{PinMetadata, PinListMetadata, MetadataKeyValues, MetadataValue};



#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
/// Used to add additional options when pinning by hash
pub struct PinOptions {
  /// multiaddresses of nodes your content is already stored on
  pub host_nodes: Option<Vec<String>>,
  /// Custom pin policy for the piece of content being pinned
  pub custom_pin_policy: Option<PinPolicy>,
  /// CID Version IPFS will use when creating a hash for your content
  pub cid_version: Option<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]

pub struct PinByJSON<S: Serialize> {
    pinata_content: S,
    pinata_options: Option<PinOptions>,
    pinata_metadata: Option<PinMetadata>,
}

