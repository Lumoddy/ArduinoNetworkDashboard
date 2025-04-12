#include <Arduino.h>

#include "result.h"
#include <SoftwareSerial.h>

#include "board_setup.cpp"

void setup()
{
    for (byte i = 0; i < softwareSerialCount; i++)
        softwareSerials[i].begin(baudRate);

    for (byte i = 0; i < inputPinCount; i++)
        pinMode(inputPins[i], INPUT);

    for (byte i = 0; i < outputPinCount; i++)
        pinMode(outputPins[i], OUTPUT);

    pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
}

// put function definitions here:
int myFunction(int x, int y) {
  return x + y;
}