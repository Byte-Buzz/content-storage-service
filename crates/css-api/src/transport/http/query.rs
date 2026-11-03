#[derive(Debug, Clone, serde::Deserialize)]
pub struct SecretQuery {
    pub expires: Option<String>,
    pub secret: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PresignedQuery {
    pub expires: Option<String>,
    pub signature: Option<String>,
}
