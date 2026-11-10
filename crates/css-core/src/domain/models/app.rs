#[derive(Debug, Clone)]
#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
pub struct App {
    pub id: i64,
    pub name: String,
}
