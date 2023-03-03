use std::collections::HashMap;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
/// Possible MetadaValues
pub enum MetadataValue {
  /// Represents a String metadata value
  String(String),
  /// Represents a float metadata value
  Float(f64),
  /// Represents an integer value
  Integer(u64),
  /// Only valid when used with a ChangePinMetadata request
  Delete,
}

/// alias type for HashMap<String, MetadataValue>
pub type MetadataKeyValues = HashMap<String, MetadataValue>;

#[derive(Debug, Serialize)]
/// Pin metadata stored along with files pinned.
pub struct PinMetadata {
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Custom name used for referencing your pinned content.
  pub name: Option<String>,
  /// List of key value items to attach with the pinned content
  pub keyvalues: MetadataKeyValues,
}


#[derive(Debug, Deserialize)]
/// Pin metadata returns from PinList query
/// This is different from [PinMetadata](struct.PinListMetadata.html) because
/// keyvalues can also be optional in this result
pub struct PinListMetadata {
  /// Custom name used for referencing your pinned content.
  pub name: Option<String>,
  /// List of key value items to attach with the pinned content
  pub keyvalues: Option<MetadataKeyValues>,
}


