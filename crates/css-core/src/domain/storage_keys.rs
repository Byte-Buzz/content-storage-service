pub const ORIGINAL_VARIANT: &str = "originals";

pub fn temp_key(app: &str, id: uuid::Uuid) -> String {
    format!("{app}/{id}")
}

pub fn variant_key(app: &str, variant: &str, id: uuid::Uuid) -> String {
    let hex = id.simple().to_string();
    format!("{app}/{variant}/{}/{}/{id}", &hex[0..2], &hex[2..4])
}
