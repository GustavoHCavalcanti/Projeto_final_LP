#include <HardwareSerial.h>

// Definição da estrutura para armazenar os dados
struct Dados {
    float TIME;
    int RPM;
};

// Configuração do Serial para o LoRa
HardwareSerial LoRaSerial(1); // UART1
#define M0 2
#define M1 4
const int AUX = 5; // Pino AUX conectado ao ESP (ajuste conforme necessário)

void setup() {
  pinMode(M0, OUTPUT);
  pinMode(M1, OUTPUT);
  //pinMode(AUX_PIN, INPUT); 
  // Configurar para Modo Normal (M0 = LOW, M1 = LOW)
  digitalWrite(M0, LOW);
  digitalWrite(M1, LOW);

  // Inicializa as portas seriais
  Serial.begin(115200); // Debug
  LoRaSerial.begin(9600, SERIAL_8N1, 16, 17); // Configura UART1: RX2 = 16, TX2 = 17

  // Configuração do pino AUX
  pinMode(AUX, INPUT);

  Serial.println("LoRa Transmissor Iniciado!");
}

void loop() {
  // Verifica se o AUX está em nível alto (módulo está pronto)
  
  if (digitalRead(AUX) == HIGH) {
    // Cria um array de estruturas com os dados fornecidos
    struct Dados dados[] = {
        {0.02, 1970},
        {0.06, 2037},
        {0.1, 2037},
        {0.14, 1977},
        {0.18, 2021}
    };

    // Envia cada ponto de dados via LoRa
    for (int i = 0; i < 5; i++) {
        String mensagem = "TIME: " + String(dados[i].TIME) + ", RPM: " + String(dados[i].RPM);
        
        // Envia a mensagem via LoRa
        LoRaSerial.println(mensagem);
        Serial.println("Mensagem enviada: " + mensagem); // Mostra no monitor serial

        delay(1000); // Aguarda 1 segundo antes de enviar o próximo ponto
    }
  } else {
    Serial.println("AUX está em LOW. Aguardando módulo ficar pronto...");
  }

  delay(100); // Pequena espera para evitar polling muito rápido no AUX
}