#include "result.h"
#include <stddef.h>

template<typename _T>
class list
{
private:
    size_t _capacity;
    size_t _length;
    _T* _begin;

public:
    list(const size_t capacity) noexcept : _length(0)
    {
        if (capacity <= 0)
        {
            _capacity = 0;
            _begin = nullptr;
            return;
        }

        _capacity = capacity;
        _begin = new _T[capacity];
    }
    list(const size_t length, _T (*const factory)(size_t)) noexcept : list(length)
    {
        for (size_t i = 0; i < _capacity; i++)
            _begin[i] = factory(i);

        _length = length;
    }
    list(const list& original) noexcept : list(original._length)
    {
        for (size_t i = 0; i < original._length; i++)
            _begin[i] = original._begin[i];

        _length = original._length;
    }
    list(const list& original, _T (*const map)(const _T&)) noexcept : list(original.length())
    {
        for (size_t i = 0; i < _capacity; i++)
            _begin[i] = map(original._begin[i]);
    }

    size_t append(const _T& item)
    {
        _length++;

        if (_length > _capacity)
            _set_capacity(_capacity * 2);

        _begin[_length - 1] = item;

        return _length;
    }
    size_t append(const list<_T>& list)
    {
        size_t oldLength = _length;
        _length += list._length;

        if (_length > _capacity)
        {
            size_t newCapacity = _capacity;

            while (newCapacity < _length)
                newCapacity *= 2;

            _set_capacity(newCapacity);
        }

        for (size_t i = 0; i < list._length; i++)
            _begin[oldLength + i] = list._begin[i];

        return _length;
    }

    bool remove_at(const size_t index)
    {
        if (index >= _length)
            return false;

        for (size_t i = index + 1; i < _length; i++)
            _begin[i - 1] = _begin[i];

        _length--;
        _begin[_length].~_T();

        return true;
    }

    bool remove_range(const size_t start, const size_t count)
    {
        if (start >= _length || count == 0)
            return false;

        size_t end = start + count;

        if (end > _length)
            end = _length;

        for (size_t i = end; i < _length; i++)
            _begin[i - count] = _begin[i];

        for (size_t i = _length - count; i < _length; i++)
            _begin[i].~_T();

        _length -= count;

        return true;
    }

    void clear()
    {
        for (size_t i = _length; i >= 0; i--)
            _begin[i].~_T();

        _length = 0;
    }

    [[nodiscard]] result<_T&> find(const bool (*const match)(const _T&)) const
    {
        for (size_t i = 0; i < _length; i++)
            if (match(_begin[i]))
                return _begin[i];

        return result_error<>(0);
    }

    [[nodiscard]] result<size_t> find_index(const bool (*const match)(const _T&)) const
    {
        for (size_t i = 0; i < _length; i++)
            if (match(_begin[i]))
                return i;

        return result_error<>(0);
    }

    [[nodiscard]] size_t length() const { return _length }
    [[nodiscard]] size_t capacity() const { return _capacity }

    [[nodiscard]] _T* begin() { return _begin }
    [[nodiscard]] const _T* begin() const { return _begin }

    [[nodiscard]] _T* end() { return _begin + _length }
    [[nodiscard]] const _T* end() const { return _begin + _length }

    [[nodiscard]] result<_T&> at(const size_t index) { return index >= _length ? result_error<>(0) : operator[](index); }
    [[nodiscard]] result<const _T&> at(const size_t index) const { return index >= _length ? result_error<>(0) : operator[](index); }

    [[nodiscard]] _T& operator[](const size_t index) { return _begin[index]; }
    [[nodiscard]] const _T& operator[](const size_t index) const { return _begin[index]; }

private:
    bool _set_capacity(const size_t capacity)
    {
        if (capacity == _capacity)
            return false;

        _T* newBegin = new _T*[capacity];
        for (size_t i = (_capacity < capacity ? _capacity : capacity) - 1; i >= 0; i--)
            newBegin[i] = _begin[i];

        _begin = newBegin;

        return true;
    }
};