#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "processing_job_kind", rename_all = "snake_case")
)]
pub enum ProcessingJobKind {
    Ingest,
    Thumbnails,
    Frames,
    Transcode,
}
