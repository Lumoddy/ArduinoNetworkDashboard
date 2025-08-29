use core::{array, mem::{self, MaybeUninit}};

pub trait RingBufferOps<T>
{
    fn push_front(&mut self, value: T) -> Result<(), T>;

    fn override_front(&mut self, value: T);

    fn pop_back(&mut self) -> Option<T>;

    fn peek_back(&self) -> Option<&T>;
}

pub struct RingBuffer<T, const N: usize>
{
    _buffer: [mem::MaybeUninit<T>; N],
    _head: usize,
    _tail: usize,
}

impl<T, const N: usize> RingBuffer<T, N>
{
    pub fn new() -> Self
    {
        RingBuffer
        {
            _buffer: array::from_fn(|_| MaybeUninit::uninit()),
            _head: 0,
            _tail: 0,
        }
    }

    fn _wrapped_increment(&self, index: usize) -> usize
    {
        if index == N - 1 { 0 } else { index + 1 }
    }
}

impl<T, const N: usize> RingBufferOps<T> for RingBuffer<T, N>
{
    fn push_front(&mut self, value: T) -> Result<(), T>
    {
        let next_head = self._wrapped_increment(self._head);
        if next_head == self._tail
        { return Err(value) }

        self._head = next_head;
        self._buffer[self._head].write(value);
        Ok(())
    }

    fn override_front(&mut self, value: T)
    {
        self._head = self._wrapped_increment(self._head);
        if self._head == self._tail
        {
            unsafe { self._buffer[self._head].assume_init_drop(); }
            self._tail = self._wrapped_increment(self._tail);
        }

        self._buffer[self._head].write(value);
    }

    fn pop_back(&mut self) -> Option<T>
    {
        if self._head == self._tail
        { return None }

        let result = unsafe
        {
            mem::replace(
                &mut self._buffer[self._tail],
                mem::MaybeUninit::uninit()).assume_init()
        };
        self._tail = self._wrapped_increment(self._tail);
        Some(result)
    }

    fn peek_back(&self) -> Option<&T>
    {
        if self._head == self._tail
        { return None }

        Some(unsafe { self._buffer[self._tail].assume_init_ref() })
    }
}