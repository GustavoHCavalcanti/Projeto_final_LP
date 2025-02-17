#include <HardwareSerial.h>

// Configuração do Serial para o LoRa
HardwareSerial LoRaSerial(1);  // UART 1

#define M0 2
#define M1 4
//#define AUX_PIN 18

void setup() {
  // Configurar GPIOs do Módulo
  pinMode(M0, OUTPUT);
  pinMode(M1, OUTPUT);
  //pinMode(AUX_PIN, INPUT); 
  // Configurar para Modo Normal (M0 = LOW, M1 = LOW)
  digitalWrite(M0, LOW);
  digitalWrite(M1, LOW);

  // Inicializar Serial
  Serial.begin(115200);       // Debug
  LoRaSerial.begin(9600, SERIAL_8N1, 16, 17);  // RX, TX
  
  Serial.println("LoRa Receptor Iniciado!");
}

void loop() {
  // Verificar se há dados disponíveis
  if (LoRaSerial.available()) {
    String mensagem = LoRaSerial.readString();
    Serial.println("Recebido: " + mensagem);
  }
  else {
    Serial.println("Erro");
  }
  delay(100);  // Aguarde um tempo antes de verificar novamente
}
