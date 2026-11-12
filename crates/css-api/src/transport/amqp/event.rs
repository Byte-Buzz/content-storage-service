use std::str::Utf8Error;

use percent_encoding::percent_decode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename = "Records")]
    records: Vec<Record>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Record {
    data: RecordData,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecordData {
    #[serde(rename = "eventName")]
    event_name: String,
    s3: S3Entity,
}

#[derive(Deserialize, Debug, Clone)]
pub struct S3Entity {
    bucket: Bucket,
    object: Object,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bucket {
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Object {
    key: String,
}

pub struct ObjectEvent {
    pub event_name: String,
    pub bucket: String,
    pub key: String,
}

impl Event {
    pub fn into_objects(self) -> Result<Vec<ObjectEvent>, Utf8Error> {
        self.records
            .into_iter()
            .map(|v| {
                Ok(ObjectEvent {
                    key: percent_decode(v.data.s3.object.key.as_bytes())
                        .decode_utf8()?
                        .to_string(),
                    event_name: v.data.event_name,
                    bucket: v.data.s3.bucket.name,
                })
            })
            .collect()
    }
}
