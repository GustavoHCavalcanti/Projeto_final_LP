use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use handlebars::Handlebars;
use serde::Serialize;
use std::env;
use std::path::{Path,PathBuf};
use std::io::{self, Read, Write};
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use crate::{LogEntry, ler_dados_memoria, salvar_dados_memoria};


#[derive(Serialize)]
struct LoginData {}

#[derive(Serialize)]
struct TelemetryData {}

// Endpoint para retornar os dados
async fn get_dados() -> impl Responder {
    match ler_dados_memoria() {
        Ok(dados) => HttpResponse::Ok().json(dados),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erro: {}", e)),
    }
}

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

async fn post_dados(body: web::Json<Value>) -> impl Responder {
    let file_path = format!("{}/dados/dadosmemoria.json", std::env::current_dir().unwrap().display());

    // Se o arquivo não existir, cria ele
    if !Path::new(&file_path).exists() {
        fs::File::create(&file_path).expect("Erro ao criar o arquivo");
    }

    let mut file_content = String::new();
    if let Ok(mut file) = OpenOptions::new().read(true).open(&file_path) {
        file.read_to_string(&mut file_content).unwrap_or(0);
        println!("Arquivo lido com sucesso!");
    }

    let mut data: Vec<Value> = if file_content.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
    };

    data.push(body.into_inner());

    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(&file_path) {
        file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes()).unwrap();
        println!("Arquivo escrito com sucesso!");
    }

    println!("Dados recebidos e salvos: {:?}", data);
    HttpResponse::Ok().json("Dados salvos com sucesso!")
}

async fn limpar_dados() -> impl Responder {
    let file_path = format!("{}/dados/dadosmemoria.json", std::env::current_dir().unwrap().display());

    // Escreve um array vazio no arquivo
    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(file_path) {
        file.write_all(b"[]").unwrap();
    }

    HttpResponse::Ok().json("Dados apagados com sucesso!")
}


// #[post("/dados")]
// async fn post_dados(body: web::Json<Value>) -> impl Responder {
//     println!("Recebido: {:?}", body);
//     HttpResponse::Ok().json("Dados salvos com sucesso")
// }

// HttpServer::new(move || {
//     App::new()
//         .service(
//             web::resource("/dados")
//                 .route(web::get().to(get_dados))
//                 .route(web::post().to(post_dados)) // Aqui corrigido
//         )
// })

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
            // Nova rota para fornecer os dados em JSON:
            .service(
                web::resource("/dados")
                    .route(web::get().to(get_dados))
                    .route(web::post().to(post_dados)) // Aqui corrigido
                    .route(web::delete().to(limpar_dados))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}