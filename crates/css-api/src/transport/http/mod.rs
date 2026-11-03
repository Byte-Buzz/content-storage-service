use actix_web::web;

mod error;
mod files;
mod query;
mod uploads;

pub fn create_http_server(cfg: &mut web::ServiceConfig) {
    cfg.service(uploads::upload_file).service(files::get_file);
}
