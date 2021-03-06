use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {}
