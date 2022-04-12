extern crate serde;
extern crate schemafy_core;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

schemafy::schemafy!(
    "xray-segmentdocument-schema-v1.0.0.json"
);
