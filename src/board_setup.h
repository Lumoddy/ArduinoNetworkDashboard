#include <Arduino.h>
#include <SoftwareSerial.h>

#include "build_options.h"

constexpr byte isAnalogInBit = (sizeof(unsigned short) * 8) - 1;
constexpr byte isAnalogOutBit = (sizeof(unsigned short) * 8) - 2;
constexpr byte isDigitalInBit = (sizeof(unsigned short) * 8) - 3;
constexpr byte isDigitalOutBit = (sizeof(unsigned short) * 8) - 4;
constexpr unsigned short pinNumberBitMask = 0xFFFF >> 4;

constexpr long baudRate = 9600;

#ifdef IS_UNO_R3
SoftwareSerial primarySerial = SoftwareSerial(0, 1);

constexpr unsigned short usablePins[] =
{
    2  | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    3  | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    4  | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    5  | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    6  | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    7  | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    8  | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    9  | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    10 | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    11 | bit(isDigitalOutBit) | bit(isDigitalInBit) | bit(isAnalogOutBit) | bit(isAnalogInBit),
    12 | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    13 | bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) &~bit(isAnalogInBit),
    A0 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
    A1 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
    A2 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
    A3 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
    A4 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
    A5 &~bit(isDigitalOutBit) | bit(isDigitalInBit) &~bit(isAnalogOutBit) | bit(isAnalogInBit),
};
constexpr byte usablePinCount = sizeof(usablePins) / sizeof(*usablePins);
#endif