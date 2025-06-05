#ifndef shared_h
#define shared_h

#include <stdlib.h>
#include "./optional.h"

template<typename _T>
class Shared
{
private:
    void *_heap;

public:
    /// @attention Stores `nullptr`, not recommended.
    constexpr Shared() noexcept : _heap(nullptr) { }
    Shared(const _T &value) noexcept : _heap(malloc(sizeof(size_t) + sizeof(_T)))
    {
        *_heapCounter(_heap) = 1;
        new (_heapValue(_heap)) _T(value);
    }
    Shared(_T &&value) noexcept : _heap(malloc(sizeof(size_t) + sizeof(_T)))
    {
        *_heapCounter(_heap) = 1;
        new (_heapValue(_heap)) _T(value);
    }
    Shared(const Shared<_T> &original) noexcept : _heap(original._heap)
    {
        if (_heap != nullptr)
            ++*_heapCounter(_heap);
    }
    Shared(Shared<_T> &&original) noexcept : _heap(original._heap)
    {
        original._heap = nullptr;
    }

    ~Shared() noexcept
    {
        if (_heap != nullptr && --*_heapCounter(_heap) == 0)
            delete _heap;
    }

    [[nodiscard]] Result<_T &> value() noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        return *_heapValue(_heap);
    }
    [[nodiscard]] Result<const _T &> value() const noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        return *_heapValue(_heap);
    }

    Shared<_T> &operator=(const Shared<_T> &other) noexcept
    {
        if (other._heap == _heap)
            return *this;

        if (_heap != nullptr && --*_heapCounter(_heap) == 0)
            delete _heap;

        _heap = other._heap;
        if (_heap != nullptr)
            ++*_heapCounter(_heap);

        return *this;
    }
    Shared<_T> &operator=(Shared<_T> &&other) noexcept
    {
        if (_heap != nullptr && --*_heapCounter(_heap) == 0)
            delete _heap;

        _heap = other._heap;
        other._heap = nullptr;

        return *this;
    }

    [[nodiscard]] bool operator==(NoneKeyword) const noexcept { return _heap == nullptr; }
    [[nodiscard]] bool operator!=(NoneKeyword) const noexcept { return _heap != nullptr; }

    [[nodiscard]] _T &operator*() noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        return *_heapValue(_heap);
    }
    [[nodiscard]] const _T &operator*() const noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        return *_heapValue(_heap);
    }

    [[nodiscard]] _T *operator->() noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        return _heapValue(_heap);
    }
    [[nodiscard]] const _T *operator->() const noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        return _heapValue(_heap);
    }

    [[nodiscard]] bool operator!() const noexcept { return _heap == nullptr; }
    [[nodiscard]] operator bool() const noexcept { return _heap != nullptr; }

private:
    [[nodiscard]] static size_t *_heapCounter(void *heap) noexcept { return static_cast<size_t *>(heap); }
    [[nodiscard]] static const size_t *_heapCounter(const void *heap) noexcept { return static_cast<const size_t *>(heap); }

    [[nodiscard]] static _T *_heapValue(void *heap) noexcept { return static_cast<_T *>(heap + sizeof(size_t)); }
    [[nodiscard]] static const _T *_heapValue(const void *heap) noexcept { return static_cast<const _T *>(heap + sizeof(size_t)); }
};

#endif