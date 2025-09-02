use core::ops::{Index, IndexMut};

use super::{PinPortID, SchedulerTaskContext};

pub struct SchedulerAllocation<const TASK_CAPACITY: usize>
{
    _tasks: heapless::Vec<(PinPortID, fn(SchedulerTaskContext)), TASK_CAPACITY>,
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub fn len(&self) -> usize { self._tasks.len() }
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub const fn new() -> Self { Self { _tasks: heapless::Vec::new() } }
}

impl<const TASK_CAPACITY: usize> Default for SchedulerAllocation<TASK_CAPACITY>
{
    fn default() -> Self { Self::new() }
}

pub trait SchedulerAllocationOps
    : IndexMut<usize, Output = (PinPortID, fn(SchedulerTaskContext))>
{
    fn can_schedule_task(&self) -> bool;

    fn schedule_task(&mut self, pin: PinPortID, task: fn(SchedulerTaskContext))
        -> Result<(), ()>;

    fn len(&self) -> usize;

    fn remove(&mut self, index: usize) -> (PinPortID, fn(SchedulerTaskContext));
}

impl<const TASK_CAPACITY: usize>
    Index<usize> for SchedulerAllocation<TASK_CAPACITY>
{
    type Output = (PinPortID, fn(SchedulerTaskContext));

    fn index(&self, index: usize) -> &Self::Output
    {
        self._tasks.index(index)
    }
}

impl<const TASK_CAPACITY: usize>
    IndexMut<usize> for SchedulerAllocation<TASK_CAPACITY>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        self._tasks.index_mut(index)
    }
}

impl<const TASK_CAPACITY: usize>
    SchedulerAllocationOps for SchedulerAllocation<TASK_CAPACITY>
{
    fn can_schedule_task(&self) -> bool
    {
        self._tasks.len() != self._tasks.capacity()
    }

    fn schedule_task(&mut self, pin: PinPortID, task: fn(SchedulerTaskContext))
        -> Result<(), ()>
    {
        self._tasks.push((pin, task)).map_err(|_| ())
    }

    fn len(&self) -> usize { self.len() }

    fn remove(&mut self, index: usize) -> (PinPortID, fn(SchedulerTaskContext))
    {
        self._tasks.remove(index)
    }
}