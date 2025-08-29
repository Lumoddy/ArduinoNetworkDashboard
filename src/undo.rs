use core::marker::PhantomData;

pub struct Undo<From, To, DeferToFn = (), DeferFromFn = ()>
{
    _undo_original: From,
    _undo_split: PhantomData<To>,
    _defer_split: DeferToFn,
    _defer_original: DeferFromFn,
}

impl<From, To> Undo<From, To>
{
    pub fn change<MapFn: FnOnce(From) -> To>(
        value: From,
        map: MapFn) -> (To, Self)
    {
        let copy = unsafe { (&raw const value).read() };

        (
            map(value),
            Undo
            {
                _undo_original: copy,
                _undo_split: PhantomData,
                _defer_split: (),
                _defer_original: (),
            },
        )
    }

    pub unsafe fn undo(self, _split: To) -> From { self._undo_original }
}

impl<From, To, DeferToFn: FnOnce(To), DeferFromFn: FnOnce(From) -> From>
    Undo<From, To, DeferToFn, DeferFromFn>
{
    pub fn change_defer(
        value: From,
        map: impl FnOnce(From) -> To,
        defer_to: DeferToFn,
        defer_from: DeferFromFn) -> (To, Self)
    {
        let copy = unsafe { (&raw const value).read() };

        (
            map(value),
            Undo
            {
                _undo_original: copy,
                _undo_split: PhantomData,
                _defer_split: defer_to,
                _defer_original: defer_from,
            },
        )
    }

    pub unsafe fn undo(self, split: To) -> From
    {
        (self._defer_split)(split);
        (self._defer_original)(self._undo_original)
    }
}