#ifndef error_types_h
#define error_types_h

struct Error
{
public:
    const char* type;

public:
    Error() noexcept : type(nullptr) { }
    Error(const char *const type) noexcept : type(type) { }
    Error(const Error &original) noexcept : type(original.type) { }

    [[nodiscard]] bool is(const Error& other) noexcept { return operator>=(other); }
    [[nodiscard]] bool is(const char *const other) noexcept { return operator>=(other); }

    Error &operator=(const Error &other) noexcept { type = other.type; return *this; }
    Error &operator=(const char *const other) noexcept { type = other; return *this; }

    [[nodiscard]] bool operator==(const Error &other) const noexcept { return type == other.type; }
    [[nodiscard]] bool operator==(const char *const other) const noexcept { return type == other; }

    [[nodiscard]] bool operator>(const Error &other) const noexcept { return operator>(other.type); }
    [[nodiscard]] bool operator>(const char *const other) const noexcept { return _contains(type, other); }
    [[nodiscard]] bool operator>=(const Error &other) const noexcept { return operator==(other) || operator>(other); }
    [[nodiscard]] bool operator>=(const char *const other) const noexcept { return operator==(other) || operator>(other); }
    [[nodiscard]] bool operator<(const Error &other) const noexcept { return operator<(other.type); }
    [[nodiscard]] bool operator<(const char *const other) const noexcept { return _contains(other, type); }
    [[nodiscard]] bool operator<=(const Error &other) const noexcept { return operator==(other) || operator<(other); }
    [[nodiscard]] bool operator<=(const char *const other) const noexcept { return operator==(other) || operator<(other); }

private:
    [[nodiscard]] static bool _contains(const char *larger, const char *smaller) noexcept
    {
        if (larger[0] != '\0' && larger[0] == '!' && larger[1] == '\0')
            return true;

        const char *const largerStart = larger;
        const char *const smallerStart = smaller;

        if (larger == nullptr || smaller == nullptr || larger == smaller)
            return false;

        while (*larger != '\0')
        {
            if (*smaller == '\0')
                return true;

            if (*smaller != *larger)
                break;

            larger++;
            smaller++;
        }

        while (*larger != '\0')
            larger++;

        while (*smaller != '\0')
            smaller++;

        larger--;
        smaller--;
        while (larger != largerStart)
        {
            if (smaller == smallerStart)
                return true;

            if (*smaller != *larger)
                break;

            larger--;
            smaller--;
        }

        return false;
    }
};

struct ErrorTypes
{
public:
    static const Error Any;
    static const Error Unknown;

    static const Error ResultNotValue;
    static const Error ResultNotError;

    static const Error Argument;

    static const Error OutOfRange;
    static const Error IndexOutOfRange;
    static const Error ArgumentOutOfRange;

    static const Error NotFound;
    static const Error ArgumentNotFound;

    static const Error NullPointer;
    static const Error ArgumentNullPointer;

    static const Error Invalid;
    static const Error ArgumentInvalid;

    static const Error Overflow;
    static const Error HeapOverflow;
    static const Error StackOverflow;

    static const Error InvalidOperation;
    static const Error InvalidFormat;
    static const Error InvalidSyntax;
    static const Error InvalidType;

    static const Error NotImplemented;
    static const Error NotSupported;
};

const Error ErrorTypes::Any = "!";
const Error ErrorTypes::Unknown = "?";

const Error ErrorTypes::ResultNotValue = "RslNot";
const Error ErrorTypes::ResultNotError = "RslNot";

const Error ErrorTypes::Argument = "Arg";

const Error ErrorTypes::OutOfRange = "OOR";
const Error ErrorTypes::IndexOutOfRange = "IdxOOR";
const Error ErrorTypes::ArgumentOutOfRange = "ArgOOR";

const Error ErrorTypes::NotFound = "NotFnd";
const Error ErrorTypes::ArgumentNotFound = "ArgNotFnd";

const Error ErrorTypes::NullPointer = "Null";
const Error ErrorTypes::ArgumentNullPointer = "ArgNull";

const Error ErrorTypes::Invalid = "Inv";
const Error ErrorTypes::ArgumentInvalid = "ArgInv";

const Error ErrorTypes::Overflow = "Ovf";
const Error ErrorTypes::HeapOverflow = "HeapOvf";
const Error ErrorTypes::StackOverflow = "StackOvf";

const Error ErrorTypes::InvalidOperation = "InvOp";
const Error ErrorTypes::InvalidFormat = "InvFmt";
const Error ErrorTypes::InvalidSyntax = "InvSytx";
const Error ErrorTypes::InvalidType = "InvType";

const Error ErrorTypes::NotImplemented = "NotImpl";
const Error ErrorTypes::NotSupported = "NotSupt";

#include <Arduino.h>
#include "def.h"

#define fail(...) fail_with_location(__VA_ARGS__, __FILE__ ":" _mac_stringify1(__LINE__))

[[noreturn]] void fail_with_location(const char *const message, const char *const location)
{
    if (message == nullptr)
        Serial.println("Failed");
    else
    {
        Serial.print("Failed: ");
        Serial.print(message);
        Serial.println();
    }

    while (true);
}
[[noreturn]] void fail_with_location(const Error &error, const char *const location)
{
    fail_with_location(error.type, location);
}

[[noreturn]] void fail_without_location(const char *const message = nullptr)
{
    if (message == nullptr)
        Serial.println("Failed");
    else
    {
        Serial.print("Failed: ");
        Serial.print(message);
        Serial.println();
    }

    while (true);
}
[[noreturn]] void fail_without_location(const Error &error)
{
    fail_without_location(error.type);
}

#include "./result.h"

#endif