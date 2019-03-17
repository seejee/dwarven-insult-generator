use actix_web::server;

mod insults;
mod web;

fn main() {
    server::new(|| {
        let config = insults::Config::from_files();
        web::build_app(config)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
}
