use serde::Deserialize; // Importa as ferramentas para deserializar dados de CSV para structs
use csv::ReaderBuilder; // Biblioteca para trabalhar com arquivos csv
use std::error::Error; // Para gerenciar erros de forma robusta

#[derive(Debug, Deserialize)]
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


// Função para ler o arquivo CSV e retornar os dados como um vetor de LogEntry
fn read_csv(file_path: &str) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    // Cria o leitor de CSV configurado para usar cabeçalhos
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Indica que o CSV tem cabeçalhos
        .from_path(file_path)?;

    let mut data = Vec::new(); // Vetor para armazenar os dados lidos
    for result in rdr.deserialize() {
        let record: LogEntry = result?; // Converte cada linha do CSV para LogEntry
        data.push(record); // Adiciona ao vetor
    }

    Ok(data) // Retorna o vetor com os dados lidos
}

// Função principal que executa o programa
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "dados.csv/dados1.csv"; // Defina o caminho para o arquivo CSV

    // Tenta ler os dados do arquivo CSV
    let data = read_csv(file_path)?;

    // Exibe o número total de linhas lidas
    println!("Número total de linhas lidas: {}", data.len());

    // Exibe a primeira entrada, se existir
    if let Some(first_entry) = data.get(0) {
        println!("Primeira entrada: {:?}", first_entry);
    } else {
        println!("Nenhum dado encontrado no arquivo.");
    }

    // TODO: Aqui podemos adicionar processamento ou geração de gráficos
    Ok(())
}