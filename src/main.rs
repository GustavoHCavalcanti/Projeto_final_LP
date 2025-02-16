use serde::{Deserialize, Serialize}; // Adiciona Serialize para JSON
use csv::ReaderBuilder; // Biblioteca para trabalhar com arquivos CSV
use std::{error::Error, fs, io}; // Adiciona fs e io para entrada do usuário
use plotly::{Plot, Scatter}; // Biblioteca para criar gráficos interativos

mod front;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct LogEntry {
    #[serde(rename = "TIME")]
    pub time: f64,
    #[serde(rename = "RPM")]
    pub rpm: u32,
    #[serde(rename = "TPS")]
    pub tps: f64,
    #[serde(rename = "Posição_do_acelerador")]
    pub posição_do_acelerador: f64,
    #[serde(rename = "Marcha")]
    pub marcha: u32,
    #[serde(rename = "Largada_validada")]
    pub largada_validada: String,
    #[serde(rename = "Fluxo_total_de_combustível")]
    pub fluxo_total_de_combustivel: f64,
    #[serde(rename = "Temp._do_motor")]
    pub temp_do_motor: f64,
    #[serde(rename = "Pressão_de_Óleo")]
    pub pressão_de_óleo: f64,
    #[serde(rename = "Temp._do_Ar")]
    pub temp_do_ar: f64,
    #[serde(rename = "Tensão_da_Bateria")]
    pub tensão_da_bateria: f64,
    #[serde(rename = "Pressão_do_freio")]
    pub pressão_do_freio: f64,
    #[serde(rename = "Tanque")]
    pub tanque: f64,
}


// Função para ler os dados do arquivo 'dadosmemoria.json'
pub fn ler_dados_memoria() -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let file_path = "dados/dadosmemoria.json";
    // Se o arquivo não existir ou estiver vazio, retorna um vetor vazio
    let contents = fs::read_to_string(file_path).unwrap_or("[]".to_string());
    let dados: Vec<LogEntry> = serde_json::from_str(&contents)?;
    Ok(dados)
}

// Função para salvar os dados atualizados no arquivo 'dadosmemoria.json'
pub fn salvar_dados_memoria(dados: &Vec<LogEntry>) -> Result<(), Box<dyn Error>> {
    let file_path = "dados/dadosmemoria.json";
    let conteudo = serde_json::to_string_pretty(dados)?;
    fs::write(file_path, conteudo)?;
    Ok(())
}

// Função para ler CSV e retornar um vetor de LogEntry
fn read_csv(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: LogEntry = result?;
        println!("Registro lido do CSV: {:?}", record);
        data.push(record);
    }

    Ok(data)
}

// Função para ler JSON e retornar um vetor de LogEntry
fn read_json(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?; // Lê o arquivo JSON como string
    let data: Vec<LogEntry> = serde_json::from_str(&file_content)?; // Converte JSON para struct
    Ok(data)
}

// Função para exibir opções e capturar entrada do usuário
fn escolher_variavel(mensagem: &str, opcoes: &[&str]) -> String {
    loop {
        println!("{}", mensagem);
        for (i, &opcao) in opcoes.iter().enumerate() {
            println!("{} - {}", i + 1, opcao);
        }

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro ao ler entrada");

        if let Ok(indice) = escolha.trim().parse::<usize>() {
            if indice > 0 && indice <= opcoes.len() {
                return opcoes[indice - 1].to_string();
            }
        }

        println!("Escolha inválida. Tente novamente.");
    }
}

// Função para gerar gráfico baseado na escolha do usuário
fn gerar_grafico_personalizado(data: &[LogEntry], eixo_x: &str, eixo_y: &str) -> Result<(), Box<dyn Error>> {
    let valores_x: Vec<f64> = match eixo_x {
        "TIME" => data.iter().map(|d| d.time).collect(),
        "RPM" => data.iter().map(|d| d.rpm as f64).collect(),
        "TPS" => data.iter().map(|d| d.tps).collect(),
        "Posição_do_acelerador" => data.iter().map(|d| d.posição_do_acelerador).collect(),
        "Marcha" => data.iter().map(|d| d.marcha as f64).collect(),
        "Fluxo_total_de_combustível" => data.iter().map(|d| d.fluxo_total_de_combustivel).collect(),
        "Temp._do_motor" => data.iter().map(|d| d.temp_do_motor).collect(),
        "Pressão_de_Óleo" => data.iter().map(|d| d.pressão_de_óleo).collect(),
        "Temp._do_Ar" => data.iter().map(|d| d.temp_do_ar).collect(),
        "Tensão_da_Bateria" => data.iter().map(|d| d.tensão_da_bateria).collect(),
        "Pressão_do_freio" => data.iter().map(|d| d.pressão_do_freio).collect(),
        "Tanque" => data.iter().map(|d| d.tanque).collect(),
        _ => vec![],
    };

    let valores_y: Vec<f64> = match eixo_y {
        "TIME" => data.iter().map(|d| d.time).collect(),
        "RPM" => data.iter().map(|d| d.rpm as f64).collect(),
        "TPS" => data.iter().map(|d| d.tps).collect(),
        "Posição_do_acelerador" => data.iter().map(|d| d.posição_do_acelerador).collect(),
        "Marcha" => data.iter().map(|d| d.marcha as f64).collect(),
        "Fluxo_total_de_combustível" => data.iter().map(|d| d.fluxo_total_de_combustivel).collect(),
        "Temp._do_motor" => data.iter().map(|d| d.temp_do_motor).collect(),
        "Pressão_de_Óleo" => data.iter().map(|d| d.pressão_de_óleo).collect(),
        "Temp._do_Ar" => data.iter().map(|d| d.temp_do_ar).collect(),
        "Tensão_da_Bateria" => data.iter().map(|d| d.tensão_da_bateria).collect(),
        "Pressão_do_freio" => data.iter().map(|d| d.pressão_do_freio).collect(),
        "Tanque" => data.iter().map(|d| d.tanque).collect(),
        _ => vec![],
    };

    // 🔴 LOG DE DEPURAÇÃO: Verificando os valores capturados
    println!("Valores X ({}) -> {:?}", eixo_x, valores_x);
    println!("Valores Y ({}) -> {:?}", eixo_y, valores_y);

    let trace = Scatter::new(valores_x, valores_y).name(format!("{} vs {}", eixo_x, eixo_y));
    let mut plot = Plot::new();
    plot.add_trace(trace);

    fs::create_dir_all("graficos")?;
    let caminho = format!("graficos/{}_vs_{}.html", eixo_x.replace(" ", "_"), eixo_y.replace(" ", "_"));
    plot.write_html(&caminho);

    println!("Gráfico gerado: {}", caminho);
    Ok(())
}


// Função para detectar a extensão do arquivo e chamar a leitura correta
fn carregar_dados(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    if file_path.ends_with(".csv") {
        println!("Detectado arquivo CSV.");
        read_csv(file_path)
    } else if file_path.ends_with(".json") {
        println!("Detectado arquivo JSON.");
        read_json(file_path)
    } else {
        Err("Formato de arquivo não suportado. Use .csv ou .json".into())
    }
}

// Função principal que executa o programa

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    front::start_frontend().await
}


//fn main() -> Result<(), Box<dyn Error>> {
//    let file_path = "dados/dados1.csv"; // Modifique para testar com um arquivo JSON
//
    // Detecta o tipo do arquivo e lê os dados
//    let data = carregar_dados(file_path)?;
//
//    println!("Número total de linhas lidas: {}", data.len());
//    if let Some(first_entry) = data.get(0) {
//        println!("Primeira entrada: {:?}", first_entry);
//    }
//
    // Permitir ao usuário escolher as variáveis do eixo X e Y
//    let variaveis = [
//        "TIME", "RPM", "TPS", "Posição do Acelerador", "Ponto de Ignição",
//        "Temp. do Motor", "Temp. do Ar", "Pressão de Óleo", "Tensão da Bateria", "Pressão do Freio"
//    ];
//
//    let eixo_x = escolher_variavel("Escolha a variável do eixo X:", &variaveis);
//    let eixo_y = escolher_variavel("Escolha a variável do eixo Y:", &variaveis);

    // Gerar o gráfico personalizado
//    gerar_grafico_personalizado(&data, &eixo_x, &eixo_y)?;

//    Ok(())
//}
