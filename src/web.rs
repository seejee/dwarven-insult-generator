extern crate actix_web;

use crate::insults::{generate_insult, Config};
use actix_web::{App, HttpRequest, HttpResponse, Result};

use askama::actix_web::TemplateIntoResponse;
use askama::Template;

pub struct AppState {
    config: Config,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn index(_req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    Index.into_response()
}

#[derive(Template)]
#[template(path = "insult.html")]
struct Insult {
    insult: String,
}

fn insult(req: &HttpRequest<AppState>) -> Result<HttpResponse> {
    let config = &req.state().config;
    let insult = generate_insult(&config);

    Insult { insult: insult }.into_response()
}

pub fn build_app(config: Config) -> App<AppState> {
    App::with_state(AppState { config: config })
        .resource("/", |r| r.f(index))
        .resource("/insult", |r| r.f(insult))
}
