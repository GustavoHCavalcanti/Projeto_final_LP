use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::io;

#[derive(Serialize)]
struct LoginData {}

#[derive(Serialize)]
struct TelemetryData {}

async fn login_page(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = LoginData {};
    match hb.render("login", &data) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Erro ao renderizar template 'login': {}", e);
            HttpResponse::InternalServerError().body("Erro ao renderizar template")
        }
    }
}

async fn telemetry_page(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = TelemetryData {};
    match hb.render("telemetry", &data) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Erro ao renderizar template 'telemetry': {}", e);
            HttpResponse::InternalServerError().body("Erro ao renderizar template")
        }
    }
}

pub async fn start_frontend() -> io::Result<()> {
    let mut handlebars = Handlebars::new();

    // Obtém o caminho absoluto da pasta atual
    let current_dir = env::current_dir().unwrap();
    let templates_path = current_dir.join("templates");

    // Imprime o caminho dos templates para depuração
    println!("Templates path: {:?}", templates_path);

    // Registra os templates de um arquivo
    match handlebars.register_template_file("login", templates_path.join("login.hbs")) {
        Ok(_) => println!("Template 'login' registrado com sucesso."),
        Err(e) => {
            eprintln!("Erro ao registrar template 'login': {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }

    match handlebars.register_template_file("telemetry", templates_path.join("telemetry.hbs")) {
        Ok(_) => println!("Template 'telemetry' registrado com sucesso."),
        Err(e) => {
            eprintln!("Erro ao registrar template 'telemetry': {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .service(web::resource("/").route(web::get().to(login_page)))
            .service(web::resource("/telemetry").route(web::get().to(telemetry_page)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}