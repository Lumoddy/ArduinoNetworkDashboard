template<typename _T>
struct shared
{
private:
    struct _data
    {
    public:
        unsigned short count;
        _T value;

    public:
        _data(const _T& value) : count(1), value(value) { }
    };

private:
    _data* _pointer;

public:
    shared(const _T& value)
    {
        _pointer = new _data(value);
    }
    shared(const shared<_T>& value)
    {
        _pointer = value._pointer;
        _pointer->count++;
    }
    ~shared()
    {
        if (--_pointer->count == 0)
            delete _pointer;
    }

    shared<_T>& operator=(const shared<_T>& other)
    {
        if (other._pointer == _pointer)
            return;

        if (--_pointer->count == 0)
            delete _pointer;

        _pointer = value._pointer;
        _pointer->count++;
    }

    bool operator==(const shared<_T>& other) const { return _pointer == other._pointer }
    bool operator==(const _T*const other) const { return &_pointer->value == other }

    bool operator!=(const shared<_T>& other) const { return !(this == other) }
    bool operator!=(const _T*const other) const { return !(this == other) }

    [[nodiscard]] _T& operator->() { return operator*(); }
    [[nodiscard]] const _T& operator->() const { return operator*(); }

    [[nodiscard]] _T& operator*() { return _pointer->value; }
    [[nodiscard]] const _T& operator*() const { return _pointer->value; }
};