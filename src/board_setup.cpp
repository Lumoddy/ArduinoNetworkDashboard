#include <Arduino.h>
#include <SoftwareSerial.h>

#include "build_options.h"

constexpr byte isAnalogBit = (sizeof(byte) * 8) - 1;
constexpr byte isAnalogBitMask = bit(isAnalogBit);
constexpr byte pinNumberBitMask = ~isAnalogBitMask;

constexpr long baudRate = 9600;

#ifdef IS_UNO_R3
const byte softwareSerialCount = 2;
SoftwareSerial softwareSerials[softwareSerialCount] = {
    SoftwareSerial(A0, 0),
    SoftwareSerial(A1, 1),
};

const byte inputPinCount = 4;
const byte inputPins[inputPinCount] = {
    A2 | bit(isAnalogBit),
    A3 | bit(isAnalogBit),
    A4 | bit(isAnalogBit),
    A5 | bit(isAnalogBit),
};

const byte outputPinCount = 12;
const byte outputPins[outputPinCount] = {
    2  & ~bit(isAnalogBit),
    3  | bit(isAnalogBit),
    4  & ~bit(isAnalogBit),
    5  | bit(isAnalogBit),
    6  | bit(isAnalogBit),
    7  & ~bit(isAnalogBit),
    8  & ~bit(isAnalogBit),
    9  | bit(isAnalogBit),
    10 | bit(isAnalogBit),
    11 | bit(isAnalogBit),
    12 & ~bit(isAnalogBit),
    13 & ~bit(isAnalogBit),
};
#endif