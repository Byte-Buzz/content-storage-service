use actix_web::web;

mod uploads;

pub fn create_http_server(cfg: &mut web::ServiceConfig) {
    cfg.service(uploads::upload_file);
}
