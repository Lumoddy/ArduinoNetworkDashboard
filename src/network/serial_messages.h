#ifndef serial_message_h
#define serial_message_h

#include "../base/def.h"
#include "../base/result.h"
#include "../base/tuple.h"
#include "../base/list.h"

/// #### Requires:
/// - `_T` : Any integer, a `Tuple`, a `List` or `List<void>` for a length value.
/// #### Example
/// ```
/// SerialMessageDecoder<T> decoder;
/// while (true)
/// {
///     const int read = Serial.timedRead();
///     if (read == -1)
///         return bad ErrorTypes::NotFound;
///
///     if (decoder.write((uint8_t)read))
///         continue;
///
///     return decoder.result();
/// }
/// ```
template<typename _T>
class SerialMessageDecoder;

template<>
class SerialMessageDecoder<uint8_t>
{
private:
    uint8_t _result = 0;
    bool _isCompleted = false;

public:
    constexpr SerialMessageDecoder() noexcept { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint8_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        if (_isCompleted)
            return false;

        _result = byte;
        _isCompleted = true;
        return false;
    }

    [[nodiscard]] bool availableForWrite() const noexcept { return !_isCompleted; }

    uint8_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _isCompleted = false;
    }

    SerialMessageDecoder<uint8_t> &operator=(SerialMessageDecoder<uint8_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<int8_t> : private SerialMessageDecoder<uint8_t>
{
public:
    constexpr SerialMessageDecoder() noexcept : SerialMessageDecoder<uint8_t>() { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int8_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept { return SerialMessageDecoder<uint8_t>::write(byte); }
    [[nodiscard]] bool availableForWrite() const noexcept { return SerialMessageDecoder<uint8_t>::availableForWrite(); }
    int8_t result() const noexcept { return (int8_t)SerialMessageDecoder<uint8_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint8_t>::reset(); }

    SerialMessageDecoder<int8_t> &operator=(SerialMessageDecoder<int8_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<uint16_t>
{
private:
    uint16_t _result = 0;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint16_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        if (_shift >= 16)
            return false;

        _result |= (uint16_t)byte << _shift;
        _shift += 8;
        return _shift < 16;
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        return _shift < 16;
    }

    uint16_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _shift = 0;
    }

    SerialMessageDecoder<uint16_t> &operator=(SerialMessageDecoder<uint16_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<int16_t> : private SerialMessageDecoder<uint16_t>
{
public:
    constexpr SerialMessageDecoder() noexcept : SerialMessageDecoder<uint16_t>() { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int16_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept { return SerialMessageDecoder<uint16_t>::write(byte); }
    [[nodiscard]] bool availableForWrite() const noexcept { return SerialMessageDecoder<uint16_t>::availableForWrite(); }
    int16_t result() const noexcept { return (int16_t)SerialMessageDecoder<uint16_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint16_t>::reset(); }

    SerialMessageDecoder<int16_t> &operator=(SerialMessageDecoder<int16_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<uint32_t>
{
private:
    uint32_t _result = 0;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint32_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        if (_shift >= 32)
            return false;

        _result |= (uint32_t)byte << _shift;
        _shift += 8;
        return _shift < 32;
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        return _shift < 32;
    }

    uint32_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _shift = 0;
    }

    SerialMessageDecoder<uint32_t> &operator=(SerialMessageDecoder<uint32_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<int32_t> : private SerialMessageDecoder<uint32_t>
{
public:
    constexpr SerialMessageDecoder() noexcept : SerialMessageDecoder<uint32_t>() { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int32_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept { return SerialMessageDecoder<uint32_t>::write(byte); }
    [[nodiscard]] bool availableForWrite() const noexcept { return SerialMessageDecoder<uint32_t>::availableForWrite(); }
    int32_t result() const noexcept { return (int32_t)SerialMessageDecoder<uint32_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint32_t>::reset(); }

    SerialMessageDecoder<int32_t> &operator=(SerialMessageDecoder<int32_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<uint64_t>
{
private:
    uint64_t _result = 0;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<uint64_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        if (_shift >= 64)
            return false;

        _result |= (uint64_t)byte << _shift;
        _shift += 8;
        return _shift < 64;
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        return _shift < 64;
    }

    uint64_t result() const noexcept { return _result; }

    void reset() noexcept
    {
        _result = 0;
        _shift = 0;
    }

    SerialMessageDecoder<uint64_t> &operator=(SerialMessageDecoder<uint64_t> &&original) noexcept = default;
};

template<>
class SerialMessageDecoder<int64_t> : private SerialMessageDecoder<uint64_t>
{
public:
    constexpr SerialMessageDecoder() noexcept : SerialMessageDecoder<uint64_t>() { }
    constexpr SerialMessageDecoder(SerialMessageDecoder<int64_t> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept { return SerialMessageDecoder<uint64_t>::write(byte); }
    [[nodiscard]] bool availableForWrite() const noexcept { return SerialMessageDecoder<uint64_t>::availableForWrite(); }
    int64_t result() const noexcept { return (int64_t)SerialMessageDecoder<uint64_t>::result(); }
    void reset() noexcept { SerialMessageDecoder<uint64_t>::reset(); }

    SerialMessageDecoder<int64_t> &operator=(SerialMessageDecoder<int64_t> &&original) noexcept = default;
};

template<typename _TFirst, typename _TSecond, typename ..._TRest>
class SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>>
{
private:
    union _ValueUnion
    {
        struct T1
        {
            SerialMessageDecoder<_TFirst> decoder;
        }
        _1;
        struct T2
        {
            _TFirst first;
            SerialMessageDecoder<Tuple<_TSecond, _TRest...>> decoder;
        }
        _2;
        struct T3
        {
            Tuple<_TFirst, _TSecond, _TRest...> value;
        }
        _3;

        constexpr _ValueUnion() noexcept { }
        ~_ValueUnion() noexcept { }
    };

private:
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    SerialMessageDecoder(SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) noexcept
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (&_value._1) typename _ValueUnion::T1(
                    static_cast<typename _ValueUnion::T1 &&>(original._value._1));
                break;
            case 2:
                _state = 2;
                ::new (&_value._2) typename _ValueUnion::T2(
                    static_cast<typename _ValueUnion::T2 &&>(original._value._2));
                break;
            case 3:
                _state = 3;
                ::new (&_value._3) typename _ValueUnion::T3(
                    static_cast<typename _ValueUnion::T3 &&>(original._value._3));
                break;
        }
    }
    ~SerialMessageDecoder() noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
            case 3:
                _value._3.~T3();
                break;
        }
    }

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        switch (_state)
        {
            case 0:
            {
                ::new (&_value._1) typename _ValueUnion::T1
                {
                    SerialMessageDecoder<_TFirst>(),
                };
                _state = 1;
            }
            case 1:
            {
                if (_value._1.decoder.write(byte))
                    return true;

                _TFirst firstValue = static_cast<_TFirst &&>(
                    _value._1.decoder.result());

                _value._1.~T1();

                ::new (&_value._2) typename _ValueUnion::T2
                {
                    static_cast<_TFirst &&>(firstValue),
                    SerialMessageDecoder<Tuple<_TSecond, _TRest...>>(),
                };
                _state = 2;
                return true;
            }
            case 2:
            {
                if (_value._2.decoder.write(byte))
                    return true;

                _TFirst first = static_cast<_TFirst &&>(_value._2.first);

                Tuple<_TSecond, _TRest...> restValue = static_cast<Tuple<_TSecond, _TRest...> &&>(
                    _value._2.decoder.result());

                _value._2.~T2();

                ::new (&_value._3) typename _ValueUnion::T3
                {
                    Tuple<_TFirst, _TSecond, _TRest...>
                    {
                        static_cast<_TFirst &&>(first),
                        static_cast<Tuple<_TSecond, _TRest...> &&>(restValue),
                    },
                };
                _state = 3;
                return false;
            }
            default:
                return false;
        }
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        switch (_state)
        {
            case 0:
            case 1:
            case 2:
                return true;
            default:
                return false;
        }
    }

    Tuple<_TFirst, _TSecond, _TRest...> &result() noexcept { return _value._3.value; }
    const Tuple<_TFirst, _TSecond, _TRest...> &result() const noexcept { return _value._3.value; }

    void reset() noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
            case 3:
                _value._3.~T3();
                break;
        }
        _state = 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
            case 3:
                _value._3.~T3();
                break;
        }

        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (&_value._1) typename _ValueUnion::T1(
                    static_cast<typename _ValueUnion::T1 &&>(original._value._1));
                break;
            case 2:
                _state = 2;
                ::new (&_value._2) typename _ValueUnion::T2(
                    static_cast<typename _ValueUnion::T2 &&>(original._value._2));
                break;
            case 3:
                _state = 3;
                ::new (&_value._3) typename _ValueUnion::T3(
                    static_cast<typename _ValueUnion::T3 &&>(original._value._3));
                break;
        }
    }
};

template<typename _TFirst>
class SerialMessageDecoder<Tuple<_TFirst>>
{
private:
    union _ValueUnion
    {
        struct T1
        {
            SerialMessageDecoder<_TFirst> decoder;
        }
        _1;
        struct T2
        {
            Tuple<_TFirst> value;
        }
        _2;

        constexpr _ValueUnion() noexcept { }
        ~_ValueUnion() noexcept { }
    };

private:
    _ValueUnion _value;
    uint8_t _state = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    SerialMessageDecoder(SerialMessageDecoder<Tuple<_TFirst>> &&original) noexcept
    {
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (&_value._1) typename _ValueUnion::T1(
                    static_cast<typename _ValueUnion::T1 &&>(original._value._1));
                break;
            case 2:
                _state = 2;
                ::new (&_value._2) typename _ValueUnion::T2(
                    static_cast<typename _ValueUnion::T2 &&>(original._value._2));
                break;
        }
    }
    ~SerialMessageDecoder() noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
        }
    }

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        switch (_state)
        {
            case 0:
            {
                ::new (&_value._1) typename _ValueUnion::T1
                {
                    SerialMessageDecoder<_TFirst>(),
                };
                _state = 1;
            }
            case 1:
            {
                if (_value._1.decoder.write(byte))
                    return true;

                _TFirst firstValue = static_cast<_TFirst &&>(
                    _value._1.decoder.result());

                _value._1.~T1();

                ::new (&_value._2) typename _ValueUnion::T2
                {
                    static_cast<_TFirst &&>(firstValue),
                };
                _state = 2;
                return false;
            }
            default:
                return false;
        }
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        switch (_state)
        {
            case 0:
            case 1:
                return true;
            default:
                return false;
        }
    }

    Tuple<_TFirst> &result() noexcept { return _value._2.value; }
    const Tuple<_TFirst> &result() const noexcept { return _value._2.value; }

    void reset() noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
        }
        _state = 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<Tuple<_TFirst>> &&original) noexcept
    {
        switch (_state)
        {
            case 1:
                _value._1.~T1();
                break;
            case 2:
                _value._2.~T2();
                break;
        }
        switch (original._state)
        {
            default:
                _state = 0;
                break;
            case 1:
                _state = 1;
                ::new (&_value._1) typename _ValueUnion::T1(
                    static_cast<typename _ValueUnion::T1 &&>(_value._1));
                break;
            case 2:
                _state = 2;
                ::new (&_value._2) typename _ValueUnion::T2(
                    static_cast<typename _ValueUnion::T2 &&>(_value._2));
                break;
        }
    }
};

template<>
class SerialMessageDecoder<List<void>>
{
private:
    uint8_t _state = 0;
    size_t _length = 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    SerialMessageDecoder(SerialMessageDecoder<List<void>> &&original) noexcept = default;
    ~SerialMessageDecoder() noexcept = default;

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        switch (_state)
        {
            case 0:
            {
                if (byte == 0xFF)
                {
                    _state = 1;
                    return true;
                }
                else
                {
                    _state = 3;
                    _length = byte;
                    return false;
                }
            }
            case 1:
            {
                _length = byte;
                _state = 2;
                return true;
            }
            case 2:
            {
                _length |= byte << 8;
                _state = 3;
                return false;
            }
            default:
                return false;
        }
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        switch (_state)
        {
            case 0:
            case 1:
            case 2:
                return true;
            default:
                return false;
        }
    }

    size_t result() const noexcept { return _length; }

    void reset() noexcept
    {
        _state = 0;
        _length = 0;
    }

    SerialMessageDecoder<List<void>> &operator=(SerialMessageDecoder<List<void>> &&original) noexcept = default;
};

template<typename _T>
class SerialMessageDecoder<List<_T>>
{
private:
    union _ValueUnion
    {
        struct T1
        {
            SerialMessageDecoder<List<void>> decoder;
        }
        _1;
        struct T2
        {
            SerialMessageDecoder<_T> decoder;
        }
        _2;

        constexpr _ValueUnion() noexcept { }
        ~_ValueUnion() noexcept { }
    };

private:
    _ValueUnion _value;
    List<_T> _result;
    size_t _length = maxof(size_t) - 0;

public:
    constexpr SerialMessageDecoder() noexcept { }
    SerialMessageDecoder(SerialMessageDecoder<List<_T>> &&original) noexcept :
        _result(static_cast<List<_T> &&>(original._result)),
        _length(original._length)
    {
        switch (_length)
        {
            case maxof(size_t) - 0:
                break;
            case maxof(size_t) - 1:
                ::new (&_value._1) typename _ValueUnion::T1(static_cast<typename _ValueUnion::T1 &&>(original._value._1));
                break;
            default:
                ::new (&_value._2) typename _ValueUnion::T2(static_cast<typename _ValueUnion::T2 &&>(original._value._2));
                break;
        }
    }
    ~SerialMessageDecoder() noexcept
    {
        switch (_length)
        {
            case maxof(size_t) - 0:
                break;
            case maxof(size_t) - 1:
                _value._1.~T1();
                break;
            default:
                _value._2.~T2();
                break;
        }
    }

public:
    /// #### Returns:
    /// `true` if another write is required.
    bool write(uint8_t byte) noexcept
    {
        switch (_length)
        {
            case maxof(size_t) - 0:
            {
                ::new (&_value._1) typename _ValueUnion::T1
                {
                    SerialMessageDecoder<List<void>>(),
                };
                _length = maxof(size_t) - 1;
            }
            case maxof(size_t) - 1:
            {
                if (_value._1.decoder.write(byte))
                    return true;

                _length = _value._1.decoder.result();

                _value._1.~T1();
                ::new (&_value._2) typename _ValueUnion::T2
                {
                    SerialMessageDecoder<_T>(),
                };

                return _length != 0;
            }
            default:
            {
                if (_result.length() >= _length)
                    return false;

                if (_value._2.decoder.write(byte))
                    return true;

                _result.add(_value._2.decoder.result());
                _value._2.decoder.reset();

                return _result.length() < _length;
            }
        }
    }

    [[nodiscard]] bool availableForWrite() const noexcept
    {
        return _result.length() < _length;
    }

    List<_T> &result() noexcept { return _result; }
    const List<_T> &result() const noexcept { return _result; }

    void reset() noexcept
    {
        switch (_length)
        {
            case maxof(size_t) - 0:
                break;
            case maxof(size_t) - 1:
                _value._1.~T1();
                break;
            default:
                _value._2.~T2();
                break;
        }

        _result.clear();
        _length = maxof(size_t) - 0;
    }

    SerialMessageDecoder &operator=(SerialMessageDecoder<List<_T>> &&original) noexcept
    {
        switch (_length)
        {
            case maxof(size_t) - 0:
                break;
            case maxof(size_t) - 1:
                _value._1.~T1();
                break;
            default:
                _value._2.~T2();
                break;
        }

        _result = static_cast<List<_T> &&>(original._result);
        _length = original._length;

        switch (original._length)
        {
            case maxof(size_t) - 0:
                break;
            case maxof(size_t) - 1:
                ::new (&_value._1) typename _ValueUnion::T1(static_cast<typename _ValueUnion::T1 &&>(original._value._1));
                break;
            default:
                ::new (&_value._2) typename _ValueUnion::T2(static_cast<typename _ValueUnion::T2 &&>(original._value._2));
                break;
        }
    }
};

/// #### Requires:
/// - `_T` : Any integer, a `Tuple`, a `List` or `List<void>` for a length value.
/// #### Example
/// ```
/// SerialMessageEncoder<T> encoder(value);
/// while (true)
/// {
///     int read = encoder.read();
///     if (read == -1)
///         break;
///
///     something((uint8_t)read);
/// }
/// ```
template<typename _T>
class SerialMessageEncoder;

template<>
class SerialMessageEncoder<uint8_t>
{
private:
    uint8_t _value;
    bool _hasRead = false;

public:
    constexpr SerialMessageEncoder(const uint8_t value) noexcept : _value(value) { }
    SerialMessageEncoder(SerialMessageEncoder<uint8_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        if (_hasRead)
            return -1;

        _hasRead = true;
        return _value;
    }

    [[nodiscard]] bool available() const noexcept { return !_hasRead; }

    void reset() noexcept
    {
        _hasRead = false;
    }
    void reset(const uint8_t newValue) noexcept
    {
        reset();
        _value = newValue;
    }

    SerialMessageEncoder<uint8_t> &operator=(SerialMessageEncoder<uint8_t> &&original) noexcept = default;
};

template<>
class SerialMessageEncoder<int8_t> : private SerialMessageEncoder<uint8_t>
{
public:
    constexpr SerialMessageEncoder(const int8_t value) noexcept : SerialMessageEncoder<uint8_t>((uint8_t)value) { }
    SerialMessageEncoder(SerialMessageEncoder<int8_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept { return SerialMessageEncoder<uint8_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageEncoder<uint8_t>::available(); }
    void reset() { return SerialMessageEncoder<uint8_t>::reset(); }
    void reset(const int8_t newValue) { return SerialMessageEncoder<uint8_t>::reset(newValue); }

    SerialMessageEncoder<int8_t> &operator=(SerialMessageEncoder<int8_t> &&original) = default;
};

template<>
class SerialMessageEncoder<uint16_t>
{
private:
    uint16_t _value;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageEncoder(const uint16_t value) noexcept : _value(value) { }
    SerialMessageEncoder(SerialMessageEncoder<uint16_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        if (_shift == 16)
            return -1;

        uint8_t result = (uint8_t)(_value >> _shift);
        _shift += 8;
        return result;
    }

    [[nodiscard]] bool available() const noexcept { return _shift < 16; }

    void reset() noexcept
    {
        _shift = 0;
    }
    void reset(const uint16_t newValue) noexcept
    {
        reset();
        _value = newValue;
    }

    SerialMessageEncoder<uint16_t> &operator=(SerialMessageEncoder<uint16_t> &&original) noexcept = default;
};

template<>
class SerialMessageEncoder<int16_t> : private SerialMessageEncoder<uint16_t>
{
public:
    constexpr SerialMessageEncoder(const int16_t value) noexcept : SerialMessageEncoder<uint16_t>((uint16_t)value) { }
    SerialMessageEncoder(SerialMessageEncoder<int16_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept { return SerialMessageEncoder<uint16_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageEncoder<uint16_t>::available(); }
    void reset() { return SerialMessageEncoder<uint16_t>::reset(); }
    void reset(const int16_t newValue) { return SerialMessageEncoder<uint16_t>::reset(newValue); }

    SerialMessageEncoder<int16_t> &operator=(SerialMessageEncoder<int16_t> &&original) = default;
};

template<>
class SerialMessageEncoder<uint32_t>
{
private:
    uint32_t _value;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageEncoder(const uint32_t value) noexcept : _value(value) { }
    SerialMessageEncoder(SerialMessageEncoder<uint32_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        if (_shift == 32)
            return -1;

        uint8_t result = (uint8_t)(_value >> _shift);
        _shift += 8;
        return result;
    }

    [[nodiscard]] bool available() const noexcept { return _shift < 32; }

    void reset() noexcept
    {
        _shift = 0;
    }
    void reset(const uint32_t newValue) noexcept
    {
        reset();
        _value = newValue;
    }

    SerialMessageEncoder<uint32_t> &operator=(SerialMessageEncoder<uint32_t> &&original) noexcept = default;
};

template<>
class SerialMessageEncoder<int32_t> : private SerialMessageEncoder<uint32_t>
{
public:
    constexpr SerialMessageEncoder(const int32_t value) noexcept : SerialMessageEncoder<uint32_t>((uint32_t)value) { }
    SerialMessageEncoder(SerialMessageEncoder<int32_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept { return SerialMessageEncoder<uint32_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageEncoder<uint32_t>::available(); }
    void reset() { return SerialMessageEncoder<uint32_t>::reset(); }
    void reset(const int32_t newValue) { return SerialMessageEncoder<uint32_t>::reset(newValue); }

    SerialMessageEncoder<int32_t> &operator=(SerialMessageEncoder<int32_t> &&original) = default;
};

template<>
class SerialMessageEncoder<uint64_t>
{
private:
    uint64_t _value;
    uint8_t _shift = 0;

public:
    constexpr SerialMessageEncoder(const uint64_t value) noexcept : _value(value) { }
    SerialMessageEncoder(SerialMessageEncoder<uint64_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        if (_shift == 64)
            return -1;

        uint8_t result = (uint8_t)(_value >> _shift);
        _shift += 8;
        return result;
    }

    [[nodiscard]] bool available() const noexcept { return _shift < 64; }

    void reset() noexcept
    {
        _shift = 0;
    }
    void reset(const uint64_t newValue) noexcept
    {
        reset();
        _value = newValue;
    }

    SerialMessageEncoder<uint64_t> &operator=(SerialMessageEncoder<uint64_t> &&original) noexcept = default;
};

template<>
class SerialMessageEncoder<int64_t> : private SerialMessageEncoder<uint64_t>
{
public:
    constexpr SerialMessageEncoder(const int64_t value) noexcept : SerialMessageEncoder<uint64_t>((uint64_t)value) { }
    SerialMessageEncoder(SerialMessageEncoder<int64_t> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept { return SerialMessageEncoder<uint64_t>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageEncoder<uint64_t>::available(); }
    void reset() { return SerialMessageEncoder<uint64_t>::reset(); }
    void reset(const int64_t newValue) { return SerialMessageEncoder<uint64_t>::reset(newValue); }

    SerialMessageEncoder<int64_t> &operator=(SerialMessageEncoder<int64_t> &&original) = default;
};

template<typename _TFirst, typename _TSecond, typename ..._TRest>
class SerialMessageEncoder<Tuple<_TFirst, _TSecond, _TRest...>>
{
private:
    union _ValueUnion
    {
        struct T1
        {
            SerialMessageEncoder<_TFirst> encoder;
        }
        _1;
        struct T2
        {
            SerialMessageEncoder<Tuple<_TSecond, _TRest...>> encoder;
        }
        _2;

        constexpr _ValueUnion() noexcept { }
        ~_ValueUnion() noexcept { }
    };

private:
    const Tuple<_TFirst, _TSecond, _TRest...> *_value;
    _ValueUnion _stateValue;
    uint8_t _state = 0;

public:
    constexpr SerialMessageEncoder(const Tuple<_TFirst, _TSecond, _TRest...> &value) noexcept :
        _value(&value) { }
    SerialMessageEncoder(SerialMessageEncoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) noexcept :
        _value(original._value)
    {
        switch (original._state)
        {
            case 0:
                _state = 0;
            case 1:
                ::new (&_stateValue._1) typename _ValueUnion::T1(
                    static_cast<typename _ValueUnion::T1 &&>(original._stateValue._1));
                _state = 1;
            case 2:
                ::new (&_stateValue._2) typename _ValueUnion::T2(
                    static_cast<typename _ValueUnion::T2 &&>(original._stateValue._2));
                _state = 2;
            default:
                _state = 3;
        }
    }
    ~SerialMessageEncoder() noexcept
    {
        switch (_state)
        {
            case 1:
                _stateValue._1.~T1();
            case 2:
                _stateValue._2.~T2();
        }
    }

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        switch (_state)
        {
            case 0:
            {
                ::new (&_stateValue._1) typename _ValueUnion::T1
                {
                    SerialMessageEncoder<_TFirst>(at<0>(*_value)),
                };
                _state = 1;
            }
            case 1:
            {
                int read = _stateValue._1.encoder.read();
                if (read != -1)
                    return read;

                _stateValue._1.~T1();

                ::new (&_stateValue._2) typename _ValueUnion::T2
                {
                    SerialMessageEncoder<Tuple<_TSecond, _TRest...>>(
                        *static_cast<const Tuple<_TSecond, _TRest...> *>(static_cast<const void *>(_value))),
                };
                _state = 2;
            }
            case 2:
            {
                int read = _stateValue._2.encoder.read();
                if (read != -1)
                    return read;

                _stateValue._2.~T2();

                _state = 3;
            }
            default:
                return -1;
        }
    }

    [[nodiscard]] bool available() const noexcept
    {
        switch (_state)
        {
            case 0:
            case 1:
            case 2:
                return true;
            default:
                return false;
        }
    }

    void reset() noexcept
    {
        switch (_state)
        {
            case 1:
                _stateValue._1.~T1();
            case 2:
                _stateValue._2.~T2();
        }
        _state = 0;
    }
    void reset(const Tuple<_TFirst, _TSecond, _TRest...> &newValue) noexcept
    {
        reset();
        _value = &newValue;
    }

    SerialMessageEncoder<Tuple<_TFirst, _TSecond, _TRest...>> &operator=(SerialMessageEncoder<Tuple<_TFirst, _TSecond, _TRest...>> &&original) noexcept
    {
        switch (_state)
        {
            case 1:
                _stateValue._1.~T1();
            case 2:
                _stateValue._2.~T2();
        }

        switch (original._state)
        {
            case 0:
                _state = 0;
            case 1:
                ::new (&_stateValue._1) typename _ValueUnion::T1(static_cast<typename _ValueUnion::T1 &&>(original._stateValue._1));
                _state = 1;
            case 2:
                ::new (&_stateValue._2) typename _ValueUnion::T2(static_cast<typename _ValueUnion::T2 &&>(original._stateValue._2));
                _state = 2;
            default:
                _state = 3;
        }
    }
};

template<typename _TFirst>
class SerialMessageEncoder<Tuple<_TFirst>> : private SerialMessageEncoder<_TFirst>
{
public:
    constexpr SerialMessageEncoder(const Tuple<_TFirst> &value) noexcept : SerialMessageEncoder<_TFirst>(at<0>(value)) { }
    SerialMessageEncoder(SerialMessageEncoder<Tuple<_TFirst>> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept { return SerialMessageEncoder<_TFirst>::read(); }
    [[nodiscard]] bool available() const noexcept { return SerialMessageEncoder<_TFirst>::available(); }
    void reset() { return SerialMessageEncoder<_TFirst>::reset(); }
    void reset(const Tuple<_TFirst> &newValue) { return SerialMessageEncoder<_TFirst>::reset(at<0>(newValue)); }

    SerialMessageEncoder<Tuple<_TFirst>> &operator=(SerialMessageEncoder<Tuple<_TFirst>> &&original) = default;
};

template<>
class SerialMessageEncoder<List<void>>
{
private:
    size_t _length;
    uint8_t _state = 0;

public:
    constexpr SerialMessageEncoder(size_t length) noexcept : _length(length) { }
    SerialMessageEncoder(SerialMessageEncoder<List<void>> &&original) noexcept = default;
    ~SerialMessageEncoder() noexcept = default;

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        switch (_state)
        {
            case 0:
            {
                if (_length < 0xFF)
                {
                    _state = 3;
                    return _length;
                }
                else
                {
                    _state = 1;
                    return 0xFF;
                }
            }
            case 1:
            {
                _state = 2;
                return (uint8_t)(_length >> 0);
            }
            case 2:
            {
                _state = 3;
                return (uint8_t)(_length >> 8);
            }
            default:
                return -1;
        }
    }

    [[nodiscard]] bool available() const noexcept
    {
        switch (_state)
        {
            case 0:
            case 1:
            case 2:
                return true;
            default:
                return false;
        }
    }

    void reset()
    {
        _state = 0;
    }
    void reset(const size_t &newValue)
    {
        reset();
        _length = newValue;
    }

    SerialMessageEncoder<List<void>> &operator=(SerialMessageEncoder<List<void>> &&original) = default;
};

template<typename _T>
class SerialMessageEncoder<List<_T>>
{
private:
    union _ValueUnion
    {
        struct T1
        {
            SerialMessageEncoder<List<void>> encoder;
        }
        _1;
        struct T2
        {
            SerialMessageEncoder<_T> encoder;
        }
        _2;

        constexpr _ValueUnion() noexcept { }
        ~_ValueUnion() noexcept { }
    };

private:
    const List<_T> *_value;
    _ValueUnion _stateValue;
    size_t _index = maxof(size_t) - 0;

public:
    constexpr SerialMessageEncoder(const List<_T> &value) noexcept :
        _value(&value) { }
    SerialMessageEncoder(SerialMessageEncoder<List<_T>> &&original) noexcept :
        _value(original._value),
        _index(original._index)
    {
        switch (original._index)
        {
            case maxof(size_t) - 0:
            case maxof(size_t) - 1:
                break;
            default:
                ::new (&_stateValue._1) typename _ValueUnion::T1(static_cast<typename _ValueUnion::T1 &&>(original._stateValue._1));
                break;
        }
    };
    ~SerialMessageEncoder() noexcept
    {
        switch (_index)
        {
            case maxof(size_t) - 0:
            case maxof(size_t) - 1:
                break;
            default:
                _stateValue._1.~T1();
                break;
        }
    }

public:
    /// #### Returns:
    /// `uint8_t` or `-1` if the end has been reached.
    int read() noexcept
    {
        switch (_index)
        {
            case maxof(size_t) - 0:
            {
                ::new (&_stateValue._1) typename _ValueUnion::T1
                {
                    SerialMessageEncoder<List<void>>(_value->length()),
                };

                _index = maxof(size_t) - 1;
            }
            case maxof(size_t) - 1:
            {
                int read = _stateValue._1.encoder.read();
                if (read != -1)
                    return read;

                _stateValue._1.~T1();

                if (_value->length() != 0)
                {
                    ::new (&_stateValue._2) typename _ValueUnion::T2
                    {
                        SerialMessageEncoder<_T>(_value->operator[](0)),
                    };
                }

                _index = 0;
            }
            default:
            {
                if (_index >= _value->length())
                    return -1;

                int read = _stateValue._2.encoder.read();
                if (read != -1)
                    return read;

                _index++;

                if (_index >= _value->length())
                    return -1;

                _stateValue._2.encoder.reset(_value->operator[](_index));

                return _stateValue._2.encoder.read();
            }
        }
    }

    [[nodiscard]] bool available() const noexcept
    {
        switch (_index)
        {
            case maxof(size_t) - 0:
                return true;
            case maxof(size_t) - 1:
                return _value->length() != 0 || _stateValue._1.encoder.available();
            default:
            {
                size_t length = _value->length();
                if (_index < length)
                    return _index < length - 1 || _stateValue._2.encoder.available();
                else
                    return false;
            }
        }
    }

    void reset()
    {
        switch (_index)
        {
            case maxof(size_t) - 0:
            case maxof(size_t) - 1:
                break;
            default:
                _stateValue._1.~T1();
                break;
        }
        _index = maxof(size_t) - 0;
    }
    void reset(const List<_T> &newValue)
    {
        reset();
        _value = &newValue;
    }

    SerialMessageEncoder<List<_T>> &operator=(SerialMessageEncoder<List<_T>> &&original)
    {
        switch (_index)
        {
            case maxof(size_t) - 0:
            case maxof(size_t) - 1:
                break;
            default:
                _stateValue._1.~T1();
                break;
        }

        _value = original._value;
        _index = original._index;

        switch (original._index)
        {
            case maxof(size_t) - 0:
            case maxof(size_t) - 1:
                break;
            default:
                ::new (&_stateValue._1) typename _ValueUnion::T1(static_cast<typename _ValueUnion::T1 &&>(original._stateValue._1));
                break;
        }
    }
};

#endif