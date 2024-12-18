#include <ArduinoBLE.h>
#include <Serial.h>

#define SHOCK_PIN 2
#define MAX_SHOCK_DURATION 1001

BLEService shock_service("a44362c5-b709-4b41-8904-f4362031ce7e");
BLEUnsignedShortCharacteristic activate_shock("155dc6c3-99c5-4f87-aa9d-329fcfaf893b", BLERead | BLEWrite | BLEWriteWithoutResponse);
BLECharacteristic shock_loop_active("873346bd-a08b-4769-b006-4375190f6bc7", BLEWrite | BLERead, 0, 1);
BLECharacteristic cooldown("1d0edd21-dfce-4906-8a47-7cf83aef1292", BLEWrite | BLERead, 0, 2);

void shock(unsigned short duration) {
  Serial.println("Administering shock...");
  digitalWrite(SHOCK_PIN, HIGH);
  delay((unsigned long) duration);
  digitalWrite(SHOCK_PIN, LOW);
  Serial.println("Shock done");
}

void shock_loop(unsigned short duration) {
  uint8_t loop_active;
  unsigned short cooldown_ms;
  cooldown.readValue(cooldown_ms);
  shock_loop_active.readValue(loop_active);
  while (loop_active) {
    shock(duration);
    delay(cooldown);
    loop_active--;
    // shock_loop_active.readValue(loop_active);
  }
}

void phoneConnected(BLEDevice phone) {
  Serial.println("Phone connected: ");
  Serial.println(phone.address());
}

void setup() {
  pinMode(SHOCK_PIN, OUTPUT);
  digitalWrite(SHOCK_PIN, LOW);

  Serial.begin(9600);
  while (!Serial);

  if (!BLE.begin()) {
    Serial.println("BLE start fail");
    for (;;);
  }

  BLE.setLocalName("Shock Clock");
  BLE.setAdvertisedService(shock_service);

  shock_service.addCharacteristic(activate_shock);
  // shock_service.addCharacteristic(shock_loop_active);
  // shock_service.addCharacteristic(cooldown);

  BLE.addService(shock_service);

  BLE.setEventHandler(BLEConnected, phoneConnected);
  
  activate_shock.setEventHandler(BLEWritten, shockActivated);

  activate_shock.setValue(0);

  //BLE.setConnectionInterval(0, -1);
  if (!BLE.advertise()) {
    Serial.println("BLE advertise fail");
    for (;;);
  }

  Serial.println("setup done");
}

void loop() {
  // put your main code here, to run repeatedly:
  BLE.poll();
  // BLEDevice phone = BLE.central();
  // if (phone) {
  //   while (phone.connected()) {

  //   }
  //   //Serial.println("Connected?");
  // }
}

void shockActivated(BLEDevice central, BLECharacteristic characteristic) {
  Serial.println("attempting shock...");
  unsigned short duration;
  characteristic.readValue(duration);
  Serial.print("Shock duration: ");
  Serial.print(duration);
  Serial.println();

  unsigned short loop_active;
  shock_loop_active.readValue(loop_active);
  if (loop_active) {
    shock_loop(duration);
  } else if (duration > 0 && duration < MAX_SHOCK_DURATION) {
    shock(duration);
  }
  characteristic.writeValue((unsigned short)0);
}