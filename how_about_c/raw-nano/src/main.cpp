#include<Arduino.h>

void setup() {
    Serial.begin(9600);
    pinMode(2,OUTPUT);
}


int count = 0;
void loop() {
    if (count % 2 == 0) {
    digitalWrite(2,HIGH);
    } else {
        digitalWrite(2,LOW);
    }
    Serial.println("h");
    delay(1000);
    if (count == 245) {
        count = 0;
    }

    count += 1;

    
}