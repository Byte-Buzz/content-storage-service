#[derive(Debug, Clone, sqlx::FromRow)]
pub struct App {
    pub id: i64,
    pub name: String,
}
