use actix_web::web;

mod files;
mod uploads;

pub fn create_http_server(cfg: &mut web::ServiceConfig) {
    cfg.service(uploads::upload_file).service(files::get_file);
}
