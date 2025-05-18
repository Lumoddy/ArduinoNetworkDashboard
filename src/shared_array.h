#ifndef shared_h
#define shared_h

#include "./optional.h"

template<typename _T>
class SharedArray
{
private:
    void *_heap;

public:
    SharedArray() noexcept : _heap(nullptr) { }
    SharedArray(const _T &value) noexcept : _heap(new _Heap(1, value)) { }
    SharedArray(_T &&value) noexcept : _heap(new _Heap(1, value)) { }
    SharedArray(const SharedArray<_T> &original) noexcept : _heap(original._heap)
    {
        if (_heap != nullptr)
            ++_heap->count;
    }
    SharedArray(SharedArray<_T> &&original) noexcept : _heap(original._heap)
    {
        original._heap = nullptr;
    }

    ~SharedArray() noexcept
    {
        if (_heap != nullptr && --_heap->count == 0)
            delete _heap;
    }

    SharedArray<_T> &operator=(const SharedArray<_T> &other) noexcept
    {
        if (_heap != nullptr && --_heap->count == 0)
            delete _heap;

        _heap = other._heap;
        if (_heap != nullptr)
            ++_heap->count;

        return *this;
    }
    SharedArray<_T> &operator=(SharedArray<_T> &&other) noexcept
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

    [[nodiscard]] _T &operator[](const size_t index) noexcept { return _elementAt(); }
    [[nodiscard]] const _T &operator[](const size_t index) const noexcept { return _elementAt(); }

    [[nodiscard]] bool operator!() const noexcept { return _heap == nullptr; }
    [[nodiscard]] operator bool() const noexcept { return _heap != nullptr; }

private:
    size_t &_heapCount() noexcept { return *((size_t *)(_heap + (0 * sizeof(size_t)))); }
    const size_t &_heapCount() const noexcept { return *((size_t *)(_heap + (0 * sizeof(size_t)))); }

    size_t &_heapLength() noexcept { return *((size_t *)(_heap + (1 * sizeof(size_t)))); }
    const size_t &_heapLength() const noexcept { return *((size_t *)(_heap + (1 * sizeof(size_t)))); }

    _T &_elementAt() noexcept { return *((_T *)(_heap + (2 * sizeof(size_t)))); }
    const _T &_elementAt() const noexcept { return *((_T *)(_heap + (2 * sizeof(size_t)))); }
};

#endif