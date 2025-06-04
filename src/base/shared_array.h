#ifndef shared_array_h
#define shared_array_h

#include "./optional.h"
#include "./index.h"

template<typename _T>
class SharedArray
{
private:
    void *_heap;

public:
    /// @attention Stores `nullptr`, not recommended.
    SharedArray() noexcept : _heap(nullptr) { }
    SharedArray(const size_t length, const _T &fillWith) noexcept : _heap(malloc((sizeof(size_t) * 2) + (sizeof(_T) * length)))
    {
        if (length == 0)
            return;

        const _T *const itEnd = _heapValueAt(_heap, length);
        for (_T *it = _heapValueAt(_heap, 0); it < itEnd; ++it)
            new (it) _T(fillWith);

        *_heapCount(_heap) = 1;
        *_heapLength(_heap) = length;
    }
    SharedArray(const size_t length, _T &&fillWith) noexcept : _heap(malloc((sizeof(size_t) * 2) + (sizeof(_T) * length)))
    {
        if (length == 0)
            return;

        const _T *const itEnd = _heapValueAt(_heap, length);
        for (_T *it = _heapValueAt(_heap, 1); it < itEnd; ++it)
            new (it) _T(static_cast<const _T &>(fillWith));

        new (_heapValueAt(0)) _T(static_cast<_T &&>(fillWith));

        *_heapCount(_heap) = 1;
        *_heapLength(_heap) = length;
    }
    /// @attention Creates uninitialized memory if `length != 0`, not recommended.
    SharedArray(const size_t length, NoneKeyword) noexcept : _heap(malloc((sizeof(size_t) * 2) + (sizeof(_T) * length))) { }
    SharedArray(const SharedArray<_T> &original) noexcept : _heap(original._heap)
    {
        if (_heap != nullptr)
            ++*_heapCount(_heap);
    }
    SharedArray(SharedArray<_T> &&original) noexcept : _heap(original._heap)
    {
        original._heap = nullptr;
    }

    ~SharedArray() noexcept
    {
        if (_heap != nullptr && --*_heapCount(_heap) == 0)
            delete _heap;
    }

    [[nodiscard]] size_t length() { return _heap == nullptr ? 0 : _heapLength(_heap); }

    [[nodiscard]] _T *begin() { return _heapValueAt(_heap, 0); }
    [[nodiscard]] const _T *begin() const { return _heapValueAt(_heap, 0); }
    [[nodiscard]] const _T *end() const { return _heapValueAt(_heap, _heapLength()); }

    SharedArray<_T> &operator=(const SharedArray<_T> &other) noexcept
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
    SharedArray<_T> &operator=(SharedArray<_T> &&other) noexcept
    {
        if (_heap != nullptr && --*_heapCounter(_heap) == 0)
            delete _heap;

        _heap = other._heap;
        other._heap = nullptr;

        return *this;
    }

    [[nodiscard]] bool operator==(NoneKeyword) const noexcept { return _heap == nullptr; }
    [[nodiscard]] bool operator!=(NoneKeyword) const noexcept { return _heap != nullptr; }

    /// #### Errors:
    /// - `ErrorTypes::NullPointer`
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(const size_t index) noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        const size_t length = *_heapLength(_heap);

        if (index >= length)
            return bad ErrorTypes::IndexOutOfRange;

        return _elementAt(index);
    }
    /// #### Errors:
    /// - `ErrorTypes::NullPointer`
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(const size_t index) const noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        const size_t length = *_heapLength(_heap);

        if (index >= length)
            return bad ErrorTypes::IndexOutOfRange;

        return _elementAt(index);
    }
    /// #### Errors:
    /// - `ErrorTypes::NullPointer`
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<_T &> at(const Index index) noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        const size_t length = *_heapLength(_heap);

        Result<size_t> actualIndex = index.actualIndex(length);
        if (!actualIndex)
            return ~actualIndex;

        return _elementAt(*actualIndex);
    }
    /// #### Errors:
    /// - `ErrorTypes::NullPointer`
    /// - `ErrorTypes::IndexOutOfRange`
    [[nodiscard]] Result<const _T &> at(const Index index) const noexcept
    {
        if (_heap == nullptr)
            return bad ErrorTypes::NullPointer;

        const size_t length = *_heapLength(_heap);

        Result<size_t> actualIndex = index.actualIndex(length);
        if (!actualIndex)
            return ~actualIndex;

        return _elementAt(*actualIndex);
    }

    [[nodiscard]] _T &operator[](const size_t index) noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        const size_t length = *_heapLength(_heap);

        if (index >= length)
            fail(ErrorTypes::IndexOutOfRange);

        return _elementAt(index);
    }
    [[nodiscard]] const _T &operator[](const size_t index) const noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        const size_t length = *_heapLength(_heap);

        if (index >= length)
            fail(ErrorTypes::IndexOutOfRange);

        return _elementAt(index);
    }
    [[nodiscard]] _T &operator[](const Index index) noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        const size_t length = *_heapLength(_heap);

        if (index.index >= length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _elementAt(length - index.index - 1) : _elementAt(index.index);
    }
    [[nodiscard]] const _T &operator[](const Index index) const noexcept
    {
        if (_heap == nullptr)
            fail(ErrorTypes::NullPointer);

        const size_t length = *_heapLength(_heap);

        if (index.index >= length)
            fail(ErrorTypes::IndexOutOfRange);

        return index.fromEnd ? _elementAt(length - index.index - 1) : _elementAt(index.index);
    }

private:
    static size_t *_heapCount(const void *const heap) noexcept { return (size_t *)heap; }
    static size_t *_heapLength(const void *const heap) noexcept { return (size_t *)(heap + sizeof(size_t)); }
    static _T *_heapValueAt(const void *const heap, const size_t index) noexcept { return (_T *)(heap + (sizeof(size_t) * 2) + (sizeof(_T) * index)); }
};

#endif