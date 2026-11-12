pub const ORIGINAL_VARIANT: &str = "originals";

pub fn temp_key(app: &str, id: uuid::Uuid) -> String {
    format!("{app}/{id}")
}

pub fn variant_key(app: &str, variant: &str, id: uuid::Uuid) -> String {
    let hex = id.simple().to_string();
    format!("{app}/{variant}/{}/{}/{id}", &hex[0..2], &hex[2..4])
}

pub fn parse_temp_key(key: &str) -> Option<(String, uuid::Uuid)> {
    let (app, id) = key.split_once('/')?;
    let id = uuid::Uuid::parse_str(id).ok()?;
    (!app.is_empty() && temp_key(app, id) == key).then(|| (app.to_string(), id))
}

#[cfg(test)]
mod tests {
    use crate::domain::storage_keys::{parse_temp_key, temp_key, variant_key};

    #[test]
    fn test_temp_key_parse() {
        let id = uuid::uuid!("e37e27ef-505d-48d1-966b-65366c2ea358");
        let app = "app";

        let key = temp_key(app, id);
        assert_eq!(parse_temp_key(&key), Some((app.into(), id)));

        let key = variant_key(app, "256x", id);
        assert_eq!(parse_temp_key(&key), None);

        let key = app;
        assert_eq!(parse_temp_key(&key), None);

        let key = format!("{}/", app);
        assert_eq!(parse_temp_key(&key), None);

        let key = format!("/{}", id);
        assert_eq!(parse_temp_key(&key), None);

        let key = format!("{app}/{}", id.simple());
        assert_eq!(parse_temp_key(&key), None);
    }
}
