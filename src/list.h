#ifndef LIST_H
#define LIST_H

#include "result.h"
#include <stddef.h>

template<typename _T>
class list
{
private:
    static constexpr size_t _MIN_CAPACITY = 4;

private:
    size_t _capacity;
    size_t _length;
    _T* _begin;

public:
    list() noexcept : list(_MIN_CAPACITY) { }
    list(const size_t capacity) noexcept : _length(0)
    {
        _capacity = capacity;
        _begin = _capacity == 0 ? nullptr : new _T[_capacity];
    }
    list(const size_t length, _T (*const factory)(const size_t)) noexcept : list(length < _MIN_CAPACITY ? _MIN_CAPACITY : length)
    {
        _length = length;

        for (size_t i = 0; i < length; i++)
            _begin[i] = factory(i);
    }
    list(const list& original) noexcept : list(original._length < _MIN_CAPACITY ? _MIN_CAPACITY : original._length)
    {
        _length = original._length;

        for (size_t i = 0; i < original._length; i++)
            _begin[i] = original._begin[i];
    }
    list(const list& original, _T (*const map)(const _T&)) noexcept : list(original._length < _MIN_CAPACITY ? _MIN_CAPACITY : original._length)
    {
        _length = original._length;

        for (size_t i = 0; i < original._length; i++)
            _begin[i] = map(original._begin[i]);
    }

    size_t append(const _T& item)
    {
        if (_length + 1 > _capacity)
            _set_capacity(_capacity * 2);

        _begin[_length] = item;

        _length++;

        return _length;
    }
    size_t append(const list<_T>& list)
    {
        if (_length + list._length > _capacity)
        {
            size_t newCapacity = _capacity;

            while (newCapacity < _length + list._length)
                newCapacity *= 2;

            _set_capacity(newCapacity);
        }

        for (size_t i = 0; i < list._length; i++)
            _begin[_length + i] = list._begin[i];

        _length += list._length;

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

    [[nodiscard]] size_t length() const { return _length; }
    [[nodiscard]] size_t capacity() const { return _capacity; }

    [[nodiscard]] _T* begin() { return _begin; }
    [[nodiscard]] const _T* begin() const { return _begin; }

    [[nodiscard]] _T* end() { return _begin + _length; }
    [[nodiscard]] const _T* end() const { return _begin + _length; }

    [[nodiscard]] result<_T&> at(const size_t index) { return index >= _length ? result_error<>(0) : operator[](index); }
    [[nodiscard]] result<const _T&> at(const size_t index) const { return index >= _length ? result_error<>(0) : operator[](index); }

    [[nodiscard]] _T& operator[](const size_t index) { return _begin[index]; }
    [[nodiscard]] const _T& operator[](const size_t index) const { return _begin[index]; }

private:
    bool _set_capacity(const size_t capacity)
    {
        if (capacity == _capacity)
            return false;

        _T* newBegin = new _T[capacity];
        _capacity = capacity;
        for (size_t i = 0; i < _length; i++)
            newBegin[i] = _begin[i];

        if (_begin != nullptr)
            delete _begin;

        _begin = newBegin;

        return true;
    }
};

#endif