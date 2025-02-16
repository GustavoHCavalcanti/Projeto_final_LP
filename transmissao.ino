#include <HardwareSerial.h>

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
    String mensagem = "hello"; // Mensagem a ser enviada
    
    // Envia a mensagem via LoRa
    LoRaSerial.println(mensagem);
    Serial.println("Mensagem enviada: " + mensagem); // Mostra no monitor serial

    delay(1000); // Aguarda 1 segundo antes de enviar novamente
  } else {
    Serial.println("AUX está em LOW. Aguardando módulo ficar pronto...");
  }

  delay(100); // Pequena espera para evitar polling muito rápido no AUX
}
