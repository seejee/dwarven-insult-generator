use actix_web::server;
use std::env;
use std::net::SocketAddr;

mod insults;
mod web;

fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], get_server_port()));

    server::new(|| {
        let config = insults::Config::from_files();
        web::build_app(config)
    })
    .bind(addr)
    .unwrap()
    .run()
}
