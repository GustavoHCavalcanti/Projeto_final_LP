use serde::{Deserialize, Serialize}; // Adiciona Serialize para JSON
use csv::ReaderBuilder; // Biblioteca para trabalhar com arquivos CSV
use std::{error::Error, fs}; // Adiciona fs para manipulação de arquivos
use plotly::{Plot, Scatter}; // Biblioteca para criar gráficos interativos

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)] // Supressão do aviso
struct LogEntry {
    #[serde(rename = "TIME")]
    time: f64,
    #[serde(rename = "RPM")]
    rpm: u32,
    #[serde(rename = "TPS")]
    tps: f64,
    #[serde(rename = "Posição_do_acelerador")]
    posição_do_acelerador: f64,
    #[serde(rename = "Ponto_de_ignição")]
    ponto_de_ignição: f64,
    #[serde(rename = "Temp._do_motor")]
    temp_do_motor: f64,
    #[serde(rename = "Temp._do_Ar")]
    temp_do_ar: f64,
    #[serde(rename = "Pressão_de_Óleo")]
    pressão_de_óleo: f64,
    #[serde(rename = "Tensão_da_Bateria")]
    tensão_da_bateria: f64,
    #[serde(rename = "Pressão_do_freio")]
    pressão_do_freio: f64,
}

// Função para ler CSV e retornar um vetor de LogEntry
fn read_csv(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;

    let mut data = Vec::new();
    for result in rdr.deserialize() {
        let record: LogEntry = result?;
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

// Função para gerar um gráfico de RPM ao longo do tempo
fn gerar_grafico(data: &[LogEntry]) -> Result<(), Box<dyn Error>> {
    let tempo: Vec<f64> = data.iter().map(|d| d.time).collect();
    let rpm: Vec<u32> = data.iter().map(|d| d.rpm).collect();

    let trace = Scatter::new(tempo, rpm).name("RPM ao longo do tempo");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.write_html("grafico.html"); // Salva o gráfico em um arquivo HTML

    println!("Gráfico gerado: grafico.html");
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
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "dados.csv/dados1teste.json"; // Modifique para testar com um arquivo JSON

    // Detecta o tipo do arquivo e lê os dados
    let data = carregar_dados(file_path)?;

    // Exibe o número total de linhas lidas
    println!("Número total de linhas lidas: {}", data.len());

    // Exibe a primeira entrada, se existir
    if let Some(first_entry) = data.get(0) {
        println!("Primeira entrada: {:?}", first_entry);
    } else {
        println!("Nenhum dado encontrado no arquivo.");
    }

    // Gera o gráfico interativo de RPM por tempo
    gerar_grafico(&data)?;

    Ok(())
}
