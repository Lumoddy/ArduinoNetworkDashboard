#ifndef serial_message_h
#define serial_message_h

#include "../base/def.h"
#include "../base/result.h"
#include "../base/tuple.h"
#include "../base/list.h"

/// #### Requires:
/// - `_T` : Implements `int operator()() const`
template<typename _T = void>
class SerialMessageReader;

template<>
class SerialMessageReader<void>
{
public:
    virtual int read() const noexcept;
};
template<typename _T>
class SerialMessageReader : public SerialMessageReader<void>
{
private:
    const _T &_callback;

public:
    constexpr SerialMessageReader(const _T &callback) noexcept : _callback(callback) { }

public:
    int read() const noexcept override { return _callback(); }
};

struct SerialMessageDecoderError
{
public:
    Error error;
    size_t bytesReadBeforeError;

public:
    constexpr SerialMessageDecoderError(const Error error, const size_t bytesReadBeforeError) noexcept :
        error(error),
        bytesReadBeforeError(bytesReadBeforeError) { }
    constexpr SerialMessageDecoderError(const SerialMessageDecoderError &original) noexcept :
        error(original.error),
        bytesReadBeforeError(original.bytesReadBeforeError) { }
    constexpr SerialMessageDecoderError(SerialMessageDecoderError &&original) noexcept :
        error(original.error),
        bytesReadBeforeError(original.bytesReadBeforeError) { }

    SerialMessageDecoderError &operator=(const SerialMessageDecoderError &value) & noexcept
    {
        error = value.error;
        bytesReadBeforeError = value.bytesReadBeforeError;
        return *this;
    }
    SerialMessageDecoderError &operator=(SerialMessageDecoderError &&value) & noexcept
    {
        error = value.error;
        bytesReadBeforeError = value.bytesReadBeforeError;
        return *this;
    }
};

/// #### Requires:
/// - `_T` : Primitive, Tuple or List.
template<typename _T>
class SerialMessageDecoder;

template<>
class SerialMessageDecoder<uint8_t>
{
private:
    const SerialMessageReader<> *_reader;
    uint8_t _result = 0;
    bool _isCompleted = false;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept
    {
        if (_isCompleted)
            return true;

        const int byte = _reader->read();
        if (byte == -1)
            return false;

        _result = (uint8_t)byte;
        _isCompleted = true;
    }

    bool available() const noexcept { return _isCompleted; }

    uint8_t result() const noexcept { return _result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<int8_t> : private SerialMessageDecoder<uint8_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint8_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint8_t>::read(); }
    bool available() const noexcept { return SerialMessageDecoder<uint8_t>::available(); }
    int8_t result() const noexcept { return (int8_t)SerialMessageDecoder<uint8_t>::result(); }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<uint16_t>
{
private:
    const SerialMessageReader<> *_reader;
    uint16_t _result = 0;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept
    {
        switch (_state)
        {
            default:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint16_t)byte) << 0;
                _state = 1;
            }
            case 1:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint16_t)byte) << 8;
                _state = 2;
            }
            case 2:
                return true;
        }
    }

    bool available() const noexcept { return _state == 2; }

    uint16_t result() const noexcept { return _result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<int16_t> : private SerialMessageDecoder<uint16_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint16_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint16_t>::read(); }
    bool available() const noexcept { return SerialMessageDecoder<uint16_t>::available(); }
    int16_t result() const noexcept { return (int16_t)SerialMessageDecoder<uint16_t>::result(); }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<uint32_t>
{
private:
    const SerialMessageReader<> *_reader;
    uint32_t _result = 0;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept
    {
        switch (_state)
        {
            default:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint32_t)byte) << 0;
                _state = 1;
            }
            case 1:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint32_t)byte) << 8;
                _state = 2;
            }
            case 2:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint32_t)byte) << 16;
                _state = 3;
            }
            case 3:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint32_t)byte) << 24;
                _state = 4;
            }
            case 4:
                return true;
        }
    }

    bool available() const noexcept { return _state == 4; }

    uint32_t result() const noexcept { return _result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<int32_t> : private SerialMessageDecoder<uint32_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint32_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint32_t>::read(); }
    bool available() const noexcept { return SerialMessageDecoder<uint32_t>::available(); }
    int32_t result() const noexcept { return (int32_t)SerialMessageDecoder<uint32_t>::result(); }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<uint64_t>
{
private:
    const SerialMessageReader<> *_reader;
    uint64_t _result = 0;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept
    {
        switch (_state)
        {
            default:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 0;
                _state = 1;
            }
            case 1:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 8;
                _state = 2;
            }
            case 2:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 16;
                _state = 3;
            }
            case 3:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 24;
                _state = 4;
            }
            case 4:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 32;
                _state = 5;
            }
            case 5:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 40;
                _state = 6;
            }
            case 6:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 48;
                _state = 7;
            }
            case 7:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result |= ((uint64_t)byte) << 56;
                _state = 8;
            }
            case 8:
                return true;
        }
    }

    bool available() const noexcept { return _state == 8; }

    uint64_t result() const noexcept { return _result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<>
class SerialMessageDecoder<int64_t> : private SerialMessageDecoder<uint64_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint64_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint64_t>::read(); }
    bool available() const noexcept { return SerialMessageDecoder<uint64_t>::available(); }
    int64_t result() const noexcept { return (int64_t)SerialMessageDecoder<uint64_t>::result(); }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<typename _TFirst, typename _TSecond, typename ..._TRest>
class SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>>
{
private:
    union _ValueUnion
    {
        uint8_t uninitialized;
        SerialMessageDecoder<_TFirst> first;
        Tuple<SerialMessageDecoder<Tuple<_TSecond, _TRest...>>, _TFirst> second;
        Tuple<_TFirst, _TSecond, _TRest...> result;

        constexpr _ValueUnion() : uninitialized() { }
    };

private:
    const SerialMessageReader<> *_reader;
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    SerialMessageDecoder(SerialMessageDecoder &&original) : _reader(original._reader)
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                _value.first = static_cast<decltype(original._value.first) &&>(original._value.first);
                break;
            case 2:
                _state = 2;
                _value.second = static_cast<decltype(original._value.second) &&>(original._value.second);
                break;
            case 3:
                _state = 3;
                _value.result = static_cast<decltype(original._value.result) &&>(original._value.result);
                break;
        }
    }
    ~SerialMessageDecoder()
    {
        switch (_state)
        {
            case 1:
                _value.first.~SerialMessageDecoder<_TFirst>();
                break;
            case 2:
                _value.second.~Tuple<SerialMessageDecoder<Tuple<_TSecond, _TRest...>>, _TFirst>();
                break;
            case 3:
                _value.result.~Tuple<_TFirst, _TSecond, _TRest...>();
                break;
        }
    }

public:
    bool read() noexcept
    {
        switch (_state)
        {
            default:
            {
                ::new (&_value.first) decltype(_value.first)(*_reader);
                _state = 1;
            }
            case 1:
            {
                if (!_value.first.read())
                    return false;

                decltype(_value.second) newValue =
                {
                    SerialMessageDecoder<Tuple<_TSecond, _TRest...>>(*_reader),
                    _value.first.result(),
                };
                _value.first.~SerialMessageDecoder<_TFirst>();
                ::new (&_value.second) decltype(_value.second)(static_cast<decltype(_value.second) &&>(newValue));
                _state = 2;
            }
            case 2:
            {
                SerialMessageDecoder<Tuple<_TSecond, _TRest...>> &secondDecoder = _value.second.at<0>();

                if (!secondDecoder.read())
                    return false;

                decltype(_value.result) newValue = secondDecoder.result();
                _value.second.~Tuple();
                ::new (&_value.result) decltype(_value.result)(static_cast<decltype(_value.result) &&>(newValue));
                _state = 3;
            }
            case 3:
                return true;
        }
    }

    bool available() const noexcept { return _state == 3; }

    Tuple<_TFirst, _TSecond, _TRest...> &result() noexcept { return _value.result; }
    const Tuple<_TFirst, _TSecond, _TRest...> &result() const noexcept { return _value.result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<typename _TFirst>
class SerialMessageDecoder<Tuple<_TFirst>>
{
private:
    union _ValueUnion
    {
        uint8_t uninitialized;
        SerialMessageDecoder<_TFirst> first;
        Tuple<_TFirst> result;

        constexpr _ValueUnion() : uninitialized() { }
    };

private:
    const SerialMessageReader<> *_reader;
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    SerialMessageDecoder(SerialMessageDecoder &&original) : _reader(original._reader)
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                _value.first = original._value.first;
                break;
            case 2:
                _state = 2;
                _value.result = original._value.result;
                break;
        }
    }
    ~SerialMessageDecoder()
    {
        switch (original._state)
        {
            case 1:
                _value.first.~SerialMessageDecoder<_TFirst>();
                break;
            case 2:
                _value.result.~Tuple<_TFirst>();
                break;
        }
    }

public:
    bool read() noexcept
    {
        switch (_state)
        {
            default:
            {
                ::new (&_value.first) decltype(_value.first)(*_reader);
                _state = 1;
            }
            case 1:
            {
                if (!_value.first.read())
                    return false;

                decltype(_value.result) newValue = _value.first.result();
                _value.first.~SerialMessageDecoder<_TFirst>();
                ::new (&_value.result) decltype(_value.result)(static_cast<decltype(_value.result) &&>(newValue));
                _state = 2;
            }
            case 2:
                return true;
        }
    }

    bool available() const noexcept { return _state == 2; }

    Tuple<_TFirst> &result() const noexcept { return _value.result; }

    SerialMessageDecoder &operator=(SerialMessageDecoder &&original) = default;
};

template<typename _T>
class SerialMessageDecoder<List<_T>>
{
public:
    SerialMessageDecoder() = delete;
    SerialMessageDecoder(const SerialMessageDecoder &) = delete;
    SerialMessageDecoder(SerialMessageDecoder &&) = delete;

public:
    /// #### Errors:
    /// - `ErrorTypes::InvalidOperation` if the reader ends before the value can fully decode.
    static Result<List<_T>, SerialMessageDecoderError> read(const SerialMessageReader<> &reader)
    {
        size_t length;

        Result<_T, SerialMessageDecoderError> first = SerialMessageDecoder<_T>::read(reader);
        if (!first)
            return bad ~first;

        return Tuple<_T>(first);
    }
};

#endif