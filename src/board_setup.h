#ifndef board_setup_h
#define board_setup_h

#include <Arduino.h>
#include <SoftwareSerial.h>

#include "./build_options.h"
#include "./base/def.h"

constexpr uint8_t isAnalogInBit = (sizeof(unsigned short) * 8) - 1;
constexpr uint8_t isAnalogOutBit = (sizeof(unsigned short) * 8) - 2;
constexpr uint8_t isDigitalInBit = (sizeof(unsigned short) * 8) - 3;
constexpr uint8_t isDigitalOutBit = (sizeof(unsigned short) * 8) - 4;
constexpr unsigned short pinNumberBitMask = 0xFFFF >> 4;

constexpr unsigned long baudRate = 9600;

#ifdef IS_ARDUINO_UNO
SoftwareSerial primarySerial = SoftwareSerial(9, 10);

constexpr unsigned short usablePins[] =
{
    2  | bit(isDigitalOutBit)                                                                 ,
    3  | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    4  | bit(isDigitalOutBit)                                                                 ,
    5  | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    6  | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    7  | bit(isDigitalOutBit)                                                                 ,
    8  | bit(isDigitalOutBit)                                                                 ,
    9  | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    10 | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    11 | bit(isDigitalOutBit)                       | bit(isAnalogOutBit)                     ,
    12 | bit(isDigitalOutBit)                                                                 ,
    13 | bit(isDigitalOutBit)                                                                 ,
    A0                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
    A1                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
    A2                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
    A3                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
    A4                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
    A5                        | bit(isDigitalInBit)                       | bit(isAnalogInBit),
};
constexpr uint8_t usablePinCount = lengthof(usablePins);
#endif

#endif