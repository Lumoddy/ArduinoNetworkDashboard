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
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint8_t> &&original) = default;
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
        return true;
    }

    [[nodiscard]] bool available() const noexcept { return _isCompleted; }

    uint8_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _isCompleted = false;
    }

    SerialMessageDecoder<uint8_t> &operator=(SerialMessageDecoder<uint8_t> &&original) = default;
};

template<>
class SerialMessageDecoder<int8_t> : private SerialMessageDecoder<uint8_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint8_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int8_t> &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint8_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageDecoder<uint8_t>::available(); }
    int8_t result() const noexcept { return (int8_t)SerialMessageDecoder<uint8_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint8_t>::reset(); }

    SerialMessageDecoder<int8_t> &operator=(SerialMessageDecoder<int8_t> &&original) = default;
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
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint16_t> &&original) = default;
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

    [[nodiscard]] bool available() const noexcept { return _state == 2; }

    uint16_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _state = 0;
    }

    SerialMessageDecoder<uint16_t> &operator=(SerialMessageDecoder<uint16_t> &&original) = default;
};

template<>
class SerialMessageDecoder<int16_t> : private SerialMessageDecoder<uint16_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint16_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int16_t> &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint16_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageDecoder<uint16_t>::available(); }
    int16_t result() const noexcept { return (int16_t)SerialMessageDecoder<uint16_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint16_t>::reset(); }

    SerialMessageDecoder<int16_t> &operator=(SerialMessageDecoder<int16_t> &&original) = default;
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
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint32_t> &&original) = default;
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

    [[nodiscard]] bool available() const noexcept { return _state == 4; }

    uint32_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _state = 0;
    }

    SerialMessageDecoder<uint32_t> &operator=(SerialMessageDecoder<uint32_t> &&original) = default;
};

template<>
class SerialMessageDecoder<int32_t> : private SerialMessageDecoder<uint32_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint32_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int32_t> &&original) = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint32_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageDecoder<uint32_t>::available(); }
    int32_t result() const noexcept { return (int32_t)SerialMessageDecoder<uint32_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint32_t>::reset(); }

    SerialMessageDecoder<int32_t> &operator=(SerialMessageDecoder<int32_t> &&original) = default;
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
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint64_t> &&original) = default;
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

    [[nodiscard]] bool available() const noexcept { return _state == 8; }

    uint64_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _state = 0;
    }

    SerialMessageDecoder<uint64_t> &operator=(SerialMessageDecoder<uint64_t> &&original) = default;
};

template<>
class SerialMessageDecoder<int64_t> : private SerialMessageDecoder<uint64_t>
{
public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        SerialMessageDecoder<uint64_t>(reader) { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int64_t> &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept { return SerialMessageDecoder<uint64_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageDecoder<uint64_t>::available(); }
    int64_t result() const noexcept { return (int64_t)SerialMessageDecoder<uint64_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint64_t>::reset(); }

    SerialMessageDecoder<int64_t> &operator=(SerialMessageDecoder<int64_t> &&original) = default;
};

template<typename _TFirst, typename _TSecond, typename ..._TRest>
class SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>>
{
private:
    using _FirstDecoder = SerialMessageDecoder<_TFirst>;
    using _RestDecoder = SerialMessageDecoder<Tuple<_TSecond, _TRest...>>;
    using _RestDecoderAndFirstValue = Tuple<_RestDecoder, _TFirst>;
    using _ResultTuple = Tuple<_TFirst, _TSecond, _TRest...>;

private:
    union _ValueUnion
    {
        _FirstDecoder first;
        _RestDecoderAndFirstValue second;
        _ResultTuple result;

        constexpr _ValueUnion() { }
        ~_ValueUnion() { }
    };

private:
    const SerialMessageReader<> *_reader;
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    SerialMessageDecoder(SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) :
        _reader(original._reader)
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (_value.first) _FirstDecoder(static_cast<_FirstDecoder &&>(original._value.first));
                break;
            case 2:
                _state = 2;
                ::new (_value.second) _RestDecoderAndFirstValue(static_cast<_RestDecoderAndFirstValue &&>(original._value.second));
                break;
            case 3:
                _state = 3;
                ::new (_value.result) _ResultTuple(static_cast<_ResultTuple &&>(original._value.result));
                break;
        }
    }
    ~SerialMessageDecoder()
    {
        switch (_state)
        {
            case 1:
                _value.first.~_FirstDecoder();
                break;
            case 2:
                _value.second.~_RestDecoderAndFirstValue();
                break;
            case 3:
                _value.result.~_ResultTuple();
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
                ::new (&_value.first) _FirstDecoder(*_reader);
                _state = 1;
            }
            case 1:
            {
                if (!_value.first.read())
                    return false;

                _RestDecoderAndFirstValue newValue =
                {
                    SerialMessageDecoder<Tuple<_TSecond, _TRest...>>(*_reader),
                    static_cast<_TFirst &&>(_value.first.result()),
                };
                _value.first.~_FirstDecoder();
                ::new (&_value.second) _RestDecoderAndFirstValue(static_cast<_RestDecoderAndFirstValue &&>(newValue));
                _state = 2;
            }
            case 2:
            {
                _RestDecoder &secondDecoder = at<0>(_value.second);

                if (!secondDecoder.read())
                    return false;

                _ResultTuple newValue =
                {
                    static_cast<_TFirst &&>(at<1>(_value.second)),
                    static_cast<Tuple<_TSecond, _TRest...> &&>(secondDecoder.result()),
                };
                _value.second.~_RestDecoderAndFirstValue();
                ::new (&_value.result) _ResultTuple(static_cast<_ResultTuple &&>(newValue));
                _state = 3;
            }
            case 3:
                return true;
        }
    }

    [[nodiscard]] bool available() const noexcept { return _state == 3; }

    [[nodiscard]] Tuple<_TFirst, _TSecond, _TRest...> &result() noexcept { return _value.result; }
    [[nodiscard]] const Tuple<_TFirst, _TSecond, _TRest...> &result() const noexcept { return _value.result; }

    void reset() noexcept
    {
        switch (_state)
        {
            case 1:
                _value.first.~_FirstDecoder();
                break;
            case 2:
                _value.second.~_RestDecoderAndFirstValue();
                break;
            case 3:
                _value.result.~_ResultTuple();
                break;
        }
        _state = 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) = default;
};

template<typename _TFirst>
class SerialMessageDecoder<Tuple<_TFirst>>
{
private:
    using _FirstDecoder = SerialMessageDecoder<_TFirst>;
    using _ResultTuple = Tuple<_TFirst>;

private:
    union _ValueUnion
    {
        _FirstDecoder first;
        _ResultTuple result;

        constexpr _ValueUnion() { }
        ~_ValueUnion() { }
    };

private:
    const SerialMessageReader<> *_reader;
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader) { }
    SerialMessageDecoder(SerialMessageDecoder<Tuple<_TFirst>> &&original) : _reader(original._reader)
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (_value.first) _FirstDecoder(static_cast<_FirstDecoder &&>(original._value.first));
                break;
            case 2:
                _state = 2;
                ::new (_value.result) _ResultTuple(static_cast<_ResultTuple &&>(original._value.result));
                break;
        }
    }
    ~SerialMessageDecoder()
    {
        switch (_state)
        {
            case 1:
                _value.first.~_FirstDecoder();
                break;
            case 2:
                _value.result.~_ResultTuple();
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

                _ResultTuple newValue = _value.first.result();
                _value.first.~_FirstDecoder();
                ::new (&_value.result) _ResultTuple(static_cast<_ResultTuple &&>(newValue));
                _state = 2;
            }
            case 2:
                return true;
        }
    }

    [[nodiscard]] bool available() const noexcept { return _state == 2; }

    [[nodiscard]] Tuple<_TFirst> &result() noexcept { return _value.result; }
    [[nodiscard]] const Tuple<_TFirst> &result() const noexcept { return _value.result; }

    void reset() noexcept
    {
        switch (_state)
        {
            case 1:
                _value.first.~_FirstDecoder();
                break;
            case 2:
                _value.result.~_ResultTuple();
                break;
        }
        _state = 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<Tuple<_TFirst>> &&original)
    {
        switch (_state)
        {
            case 1:
                _value.first.~_FirstDecoder();
                break;
            case 2:
                _value.result.~_ResultTuple();
                break;
        }
        _reader = original._reader;
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (_value.first) _FirstDecoder(static_cast<_FirstDecoder &&>(original._value.first));
                break;
            case 2:
                _state = 2;
                ::new (_value.result) _ResultTuple(static_cast<_ResultTuple &&>(original._value.result));
                break;
        }

        return *this;
    }
};

template<typename _T>
class SerialMessageDecoder<List<_T>>
{
private:
    static constexpr size_t _indexIsStateBit = (sizeof(size_t) * 8) - 1;
    static constexpr size_t _indexIsStateBitMask = 1 << _indexIsStateBit;

private:
    const SerialMessageReader<> *_reader;
    SerialMessageDecoder<_T> _decoder;
    List<_T> _result;
    size_t _index = _indexIsStateBitMask + 0;
    uint8_t _firstHalfOfLength = 0;

public:
    constexpr SerialMessageDecoder(const SerialMessageReader<> &reader) :
        _reader(&reader),
        _decoder(reader) { }
    SerialMessageDecoder(SerialMessageDecoder<List<_T>> &&original) = default;
    ~SerialMessageDecoder() = default;

public:
    bool read() noexcept
    {
        switch (_index)
        {
            case _indexIsStateBitMask + 0:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                if (byte != 0xFF)
                {
                    _result.setCapacity(byte & 0xFF);
                    _index = 0;
                    break;
                }
            }
            case _indexIsStateBitMask + 1:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _firstHalfOfLength = byte & 0xFF;
            }
            case _indexIsStateBitMask + 2:
            {
                const int byte = _reader->read();
                if (byte == -1)
                    return false;

                _result.setCapacity(_firstHalfOfLength | ((byte & 0xFF) << 8));
                _index = 0;
                break;
            }
        }

        for (; _index < _result.length(); ++_index)
        {
            if (!_decoder.read())
                return false;

            _T newValue = static_cast<_T &&>(_decoder.result());
            _decoder.reset();

            _result.add(static_cast<_T &&>(newValue));
        }

        return true;
    }

    [[nodiscard]] bool available() const noexcept { return _index == _result.length(); }

    [[nodiscard]] List<_T> &result() noexcept { return _result; }
    [[nodiscard]] const List<_T> &result() const noexcept { return _result; }

    void reset()
    {
        _decoder.reset();
        _result.clear();
        _index = 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<List<_T>> &&original) = default;
};

#endif