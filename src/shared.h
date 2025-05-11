#ifndef shared_h
#define shared_h

#include "./optional.h"

template<typename _T>
class Shared
{
private:
    struct _Heap
    {
    public:
        size_t count;
        _T value;

    public:
        _Heap(const size_t count, const _T &value) : count(count), value(value) { }
        _Heap(const size_t count, _T &&value) : count(count), value(value) { }

        ~_Heap()
        {
            value.~_T();
        }
    };

private:
    _Heap *_heap;

public:
    Shared() noexcept : _heap(nullptr) { }
    Shared(const _T &value) noexcept : _heap(new _Heap(1, value)) { }
    Shared(_T &&value) noexcept : _heap(new _Heap(1, value)) { }
    Shared(const Shared<_T> &original) noexcept : _heap(original._heap)
    {
        if (_heap != nullptr)
            ++_heap->count;
    }
    Shared(Shared<_T> &&original) noexcept : _heap(original._heap)
    {
        original._heap = nullptr;
    }

    ~Shared() noexcept
    {
        if (_heap != nullptr && --_heap->count == 0)
            delete _heap;
    }

    Shared<_T> &operator=(const Shared<_T> &other) noexcept
    {
        if (_heap != nullptr && --_heap->count == 0)
            delete _heap;

        _heap = other._heap;
        if (_heap != nullptr)
            ++_heap->count;

        return *this;
    }
    Shared<_T> &operator=(Shared<_T> &&other) noexcept
    {
        if (_heap != nullptr && --_heap->count == 0)
            delete _heap;

        _heap = other._heap;
        other._heap = nullptr;

        return *this;
    }

    bool operator==(NoneKeyword other) const noexcept { return _heap == nullptr; }
    bool operator!=(NoneKeyword other) const noexcept { return _heap != nullptr; }

    [[nodiscard]] _T &operator*() noexcept { return _heap->value; }
    [[nodiscard]] const _T &operator*() const noexcept { return _heap->value; }

    [[nodiscard]] _T *operator->() noexcept { return &_heap->value; }
    [[nodiscard]] const _T *operator->() const noexcept { return &_heap->value; }

    [[nodiscard]] bool operator!() const noexcept { return _heap == nullptr; }
    [[nodiscard]] operator bool() const noexcept { return _heap != nullptr; }
};

#endif