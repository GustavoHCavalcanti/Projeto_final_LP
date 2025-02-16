use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_files::Files;
use handlebars::Handlebars;
use serde::{Serialize, Deserialize};
use serde::Deserializer;
use std::env;
use std::path::Path;
use std::io::{self, Read, Write};
use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use crate::{LogEntry, ler_dados_memoria, salvar_dados_memoria, carregar_dados, gerar_grafico_personalizado};

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
    let file_path = format!("{}/dados/dadosmemoria.json", env::current_dir().unwrap().display());

    if !Path::new(&file_path).exists() {
        fs::File::create(&file_path).expect("Erro ao criar o arquivo");
    }

    let mut file_content = String::new();
    if let Ok(mut file) = OpenOptions::new().read(true).open(&file_path) {
        file.read_to_string(&mut file_content).unwrap_or(0);
    }

    let mut data: Vec<Value> = if file_content.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&file_content).unwrap_or_else(|_| Vec::new())
    };

    data.push(body.into_inner());

    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(&file_path) {
        file.write_all(serde_json::to_string_pretty(&data).unwrap().as_bytes()).unwrap();
    }

    HttpResponse::Ok().json("Dados salvos com sucesso!")
}

async fn limpar_dados() -> impl Responder {
    let file_path = format!("{}/dados/dadosmemoria.json", env::current_dir().unwrap().display());

    if let Ok(mut file) = OpenOptions::new().write(true).truncate(true).open(file_path) {
        file.write_all(b"[]").unwrap();
    }

    HttpResponse::Ok().json("Dados apagados com sucesso!")
}

// ===================================
// ** NOVA FUNCIONALIDADE: SELEÇÃO DE VARIÁVEIS E GERAÇÃO DE GRÁFICO **
// ===================================

#[derive(Serialize)]
struct EscolhaVariaveis {
    variaveis: Vec<&'static str>,
    grafico_url: Option<String>,
}

#[derive(Deserialize)]
struct GraficoRequest {
    eixo_x: String,
    eixo_y: String,
    time_start: Option<f64>,
    time_end: Option<f64>,
}

async fn escolher_variaveis(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let context = EscolhaVariaveis {
        variaveis: vec![
            "TIME", "RPM", "TPS", "Posição_do_acelerador", "Marcha",
            "Largada_validada", "Fluxo_total_de_combustível", "Temp._do_motor",
            "Pressão_de_Óleo", "Temp._do_Ar", "Tensão_da_Bateria", "Pressão_do_freio", "Tanque"
        ],
        grafico_url: None,
    };

    match hb.render("escolher_variaveis", &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Erro ao renderizar template: {}", e);
            HttpResponse::InternalServerError().body("Erro ao renderizar template")
        }
    }
}

// Processa a requisição e gera o gráfico
#[post("/gerar_grafico")]
async fn gerar_grafico(
    form: web::Form<GraficoRequest>,
    hb: web::Data<Handlebars<'_>>
) -> impl Responder {
    let file_path = "dados/dados1.csv";
    let data = match carregar_dados(file_path) {
        Ok(dados) => dados,
        Err(_) => return HttpResponse::InternalServerError().body("Erro ao carregar os dados."),
    };

    // Verifica se os valores de tempo são válidos ou vazios
    let time_start = form.time_start.unwrap_or_else(|| f64::MIN);
    let time_end = form.time_end.unwrap_or_else(|| f64::MAX);

    // Filtra os dados pelo tempo apenas se o usuário inseriu valores válidos
    let dados_filtrados: Vec<LogEntry> = data
        .into_iter()
        .filter(|entry| entry.time >= time_start && entry.time <= time_end)
        .collect();

    // Verifica se há dados disponíveis após o filtro
    if dados_filtrados.is_empty() {
        return HttpResponse::BadRequest().body("Nenhum dado encontrado para o intervalo de tempo fornecido.");
    }

    // Gera o gráfico
    if let Err(e) = gerar_grafico_personalizado(&dados_filtrados, &form.eixo_x, &form.eixo_y) {
        eprintln!("Erro ao gerar gráfico: {}", e);
        return HttpResponse::InternalServerError().body("Erro ao gerar gráfico.");
    }

    // URL do gráfico gerado
    let url = format!("/graficos/{}_vs_{}.html", form.eixo_x.replace(" ", "_"), form.eixo_y.replace(" ", "_"));

    // Renderiza a página com o gráfico gerado
    let context = EscolhaVariaveis {
        variaveis: vec![
            "TIME", "RPM", "TPS", "Posição_do_acelerador", "Marcha", 
            "Largada_validada", "Fluxo_total_de_combustível", "Temp._do_motor", 
            "Pressão_de_Óleo", "Temp._do_Ar", "Tensão_da_Bateria", "Pressão_do_freio", "Tanque"
        ],
        grafico_url: Some(url),
    };

    match hb.render("escolher_variaveis", &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Erro ao renderizar template com gráfico: {}", e);
            HttpResponse::InternalServerError().body("Erro ao renderizar template com gráfico")
        }
    }
}


#[post("/reset")]
async fn reset_grafico(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let context = EscolhaVariaveis {
        variaveis: vec![
            "TIME", "RPM", "TPS", "Posição_do_acelerador", "Marcha", 
            "Largada_validada", "Fluxo_total_de_combustível", "Temp._do_motor", 
            "Pressão_de_Óleo", "Temp._do_Ar", "Tensão_da_Bateria", "Pressão_do_freio", "Tanque"
        ],
        grafico_url: None, // Remove qualquer gráfico gerado anteriormente
    };

    match hb.render("escolher_variaveis", &context) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(e) => {
            eprintln!("Erro ao renderizar template ao resetar gráfico: {}", e);
            HttpResponse::InternalServerError().body("Erro ao resetar gráfico")
        }
    }
}

// =======================
// ** ATUALIZAÇÃO NO SERVIDOR **
// =======================

pub async fn start_frontend() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    let templates_path = env::current_dir().unwrap().join("templates");

    handlebars.register_template_file("login", templates_path.join("login.hbs"))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Erro ao registrar template 'login': {}", e)))?;

    handlebars.register_template_file("telemetry", templates_path.join("telemetry.hbs"))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Erro ao registrar template 'telemetry': {}", e)))?;

    handlebars.register_template_file("escolher_variaveis", templates_path.join("escolher_variaveis.hbs"))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Erro ao registrar template 'escolher_variaveis': {}", e)))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(handlebars.clone()))
            .service(web::resource("/").route(web::get().to(login_page)))
            .service(web::resource("/telemetry").route(web::get().to(telemetry_page)))
            .service(web::resource("/escolher_variaveis").route(web::get().to(escolher_variaveis)))
            .service(gerar_grafico)
            .service(Files::new("/graficos", "graficos").show_files_listing())
            .service(reset_grafico)
            .service(
                web::resource("/dados")
                    .route(web::get().to(get_dados))
                    .route(web::post().to(post_dados))
                    .route(web::delete().to(limpar_dados))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
