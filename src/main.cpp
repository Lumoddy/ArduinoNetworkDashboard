#include <Arduino.h>
#include "./base/optional.h"
#include "./base/result.h"
#include "./base/shared.h"
#include "./base/list.h"
#include "./base/sorted_list.h"
#include "./base/tuple.h"
#include "./network/serial_messages.h"

void setup()
{
    pinMode(LED_BUILTIN, OUTPUT);
}

void show(bool success)
{
    digitalWrite(LED_BUILTIN, success ? HIGH : LOW);
    delay(700);
    digitalWrite(LED_BUILTIN, LOW);
    delay(100);
}

struct E
{
    signed char operator()(int a, int b)
    {
        return b - a;
    }
};

void loop()
{
    // Optional<A> v1 = A();
    // Optional<char> v2 = 'a';
    // Optional<char> v3 = none;
    // Optional<char &> v4 = *v2;
    // auto a = *v4;

    // Result<A, E> v1 = A();
    // Result<char, E> v2 = 'a';
    // Result<char, E> v3 = bad E();
    // auto a = *v2;

    // Shared<A> v1 = A();
    // Shared<A> v2 = v1;
    // Shared<A> v3 = v2;

    SortedList<char> sortedList = SortedList<char>();

    List<char> list = List<char>();

    list.add('|', 'a');
    list.add('|', 'a', 'a');
    list.add('|', 'a', 'a', 'a');
    list.add('|');

    List<char> clone = list;

    list.setCapacity(10);

    // Result<char> r = 'a';
    // show(r);

    // r = bad;
    // show(!r);

    // show(r == bad);

    // r = Result<char>('b');
    // show(r != bad);
    // show(r == 'b');
    // show(r != 'a');

    // show(r == Result<char>('b'));
    // show(r != Result<char>('a'));
    // show(Result<char>('a') | Result<char>('b'));
    // show(Result<char>('a') & Result<char>('b'));
    // show(!(Result<char>('a') & bad));
    // show(Result<char>(bad) | Result<char>('a'));

    // show(Result<char>(bad) | []() -> Result<char> { return 'a'; });
    // bool used = false;
    // show(Result<char>(bad) & +[&]() -> Result<char> { used = true; return 'a'; });

    digitalWrite(LED_BUILTIN, HIGH);
    delay(100);
    digitalWrite(LED_BUILTIN, LOW);
    delay(100);
    digitalWrite(LED_BUILTIN, HIGH);
    delay(100);
    digitalWrite(LED_BUILTIN, LOW);
    delay(100);
    digitalWrite(LED_BUILTIN, HIGH);
    delay(100);
    digitalWrite(LED_BUILTIN, LOW);
    delay(1100);

    SerialMessageReader<int (*)()> reader([]{ return Serial.read(); });
    SerialMessageDecoder<Tuple<int32_t, int64_t, int8_t>> decoder(reader);
    SerialMessageDecoder<List<int64_t>> asdecoder(reader);
}