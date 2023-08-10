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

// Accepting any object that can transformed into JSON format
// so that it can transferred to other protocols, as JSON is a 
// popular way to interchange data btn systems/networks

pub struct PinByJSON<S: Serialize> {
    pinata_content: S,
    pinata_option: Option<PinOptions>,
    pinata_metadata: Option<PinMetadata>,
}

impl <S> PinByJSON<S> where S: Serialize {
    pub fn new(json_data: S) -> PinByJSON<S> {
        PinByJSON {
            pinata_content: json_data,
            pinata_metadata: None,
            pinata_option: None
        }
    }

      /// Consumes the PinByHash and returns a new PinByHash with pinata options set.
  pub fn set_options(mut self, options: PinOptions) -> PinByJson<S> {
    self.pinata_option = Some(options);
    self
  }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Represents a PinnedObject
pub struct PinnedObject {
  /// IPFS multi-hash provided back for your content
  pub ipfs_hash: String,
  /// This is how large (in bytes) the content you just pinned is
  pub pin_size: u64,
  /// Timestamp for your content pinning in ISO8601 format
  pub timestamp: String
}


