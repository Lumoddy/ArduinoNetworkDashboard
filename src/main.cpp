#include <Arduino.h>
#include "./base/optional.h"
#include "./base/result.h"
#include "./base/shared.h"
#include "./base/list.h"
#include "./base/sorted_list.h"
#include "./base/tuple.h"
#include "./network/serial_messages.h"
#include "./board_setup.h"

void setup()
{
    pinMode(LED_BUILTIN, OUTPUT);
    Serial.begin(baudRate);
    primarySerial.begin(baudRate);
}

void pulse(bool success = true)
{
    digitalWrite(LED_BUILTIN, success ? HIGH : LOW);
    delay(200);
    digitalWrite(LED_BUILTIN, LOW);
    delay(100);
}

void separatePulse()
{
    digitalWrite(LED_BUILTIN, HIGH);
    delay(800);
    digitalWrite(LED_BUILTIN, LOW);
    delay(800);
}

void failPulse()
{
    digitalWrite(LED_BUILTIN, HIGH);
    delay(50);
    digitalWrite(LED_BUILTIN, LOW);
    delay(50);
    digitalWrite(LED_BUILTIN, HIGH);
    delay(50);
    digitalWrite(LED_BUILTIN, LOW);
    delay(50);
    digitalWrite(LED_BUILTIN, HIGH);
    delay(50);
    digitalWrite(LED_BUILTIN, LOW);
    delay(600);
}

int timedRead(Stream &stream)
{
    int c;
    auto _startMillis = millis();
    do
    {
        c = stream.read();
        if (c != -1)
            return c;
    }
    while(millis() - _startMillis < 1000);
    return -1;
}

void handleTestCommands()
{
    if (Serial.available())
    {
        uint8_t read = (uint8_t)Serial.read();
        switch (read)
        {
            case 'p':
            {
                pulse();
                break;
            }
            case '+':
            {
                digitalWrite(LED_BUILTIN, HIGH);

                SerialMessageDecoder<Tuple<int32_t, int32_t>> decoder;

                while (true)
                {
                    int read = timedRead(Serial);
                    if (read == -1)
                    {
                        failPulse();
                        return;
                    }

                    if (!decoder.write((uint8_t)read))
                        break;
                }

                auto input = decoder.result();

                SerialMessageEncoder<int32_t> encoder(at<0>(input) + at<1>(input));

                while (true)
                {
                    if (!encoder.available())
                        break;

                    Serial.write((uint8_t)encoder.read());
                }

                digitalWrite(LED_BUILTIN, LOW);

                break;
            }
            case 's':
            {
                digitalWrite(LED_BUILTIN, HIGH);

                SerialMessageDecoder<List<unsigned char>> decoder;

                while (true)
                {
                    int read = timedRead(Serial);
                    if (read == -1)
                    {
                        failPulse();
                        return;
                    }

                    if (!decoder.write((uint8_t)read))
                        break;
                }

                auto input = decoder.result();

                SerialMessageEncoder<List<unsigned char>> encoder(input);

                while (true)
                {
                    if (!encoder.available())
                        break;

                    Serial.write((uint8_t)encoder.read());
                }

                digitalWrite(LED_BUILTIN, LOW);

                break;
            }
            case 'r':
            {
                digitalWrite(LED_BUILTIN, HIGH);

                while (true)
                {
                    int read = timedRead(Serial);
                    if (read == -1)
                        break;

                    primarySerial.write((uint8_t)read);
                }

                digitalWrite(LED_BUILTIN, LOW);

                break;
            }
            default:
            {
                failPulse();
            }
        }
    }

    if (primarySerial.available())
    {
        digitalWrite(LED_BUILTIN, HIGH);

        while (true)
        {
            int read = timedRead(primarySerial);
            if (read == -1)
                break;

            Serial.write((uint8_t)read);
        }

        digitalWrite(LED_BUILTIN, LOW);
    }
}

void loop()
{
    handleTestCommands();
}