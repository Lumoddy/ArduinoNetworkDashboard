use core::cmp;

use super::SchedulerTaskContext;

pub struct SchedulerAllocation<const TASK_CAPACITY: usize>
{
    _tasks: heapless::BinaryHeap<
        _TaskEntry,
        heapless::binary_heap::Max,
        TASK_CAPACITY>,
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub fn len(&self) -> usize { self._tasks.len() }
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub const fn new() -> Self { Self { _tasks: heapless::BinaryHeap::new() } }
}

impl<const TASK_CAPACITY: usize> Default for SchedulerAllocation<TASK_CAPACITY>
{
    fn default() -> Self { Self::new() }
}

pub trait SchedulerAllocationOps
{
    fn can_schedule_task(&self) -> bool;

    fn push_task(
        &mut self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>;

    fn next(&mut self, current_time: u64)
        -> Option<fn(SchedulerTaskContext)>;

    fn next_task_time(&self) -> Option<u64>;
}

impl<const TASK_CAPACITY: usize>
    SchedulerAllocationOps for SchedulerAllocation<TASK_CAPACITY>
{
    fn can_schedule_task(&self) -> bool
    {
        self._tasks.len() != self._tasks.capacity()
    }

    fn push_task(
        &mut self,
        priority: u8,
        cycles_after_init: u64,
        task: fn(SchedulerTaskContext)) -> Result<(), ()>
    {
        match self._tasks.push(_TaskEntry { priority, cycles_after_init, task })
        {
            Ok(()) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn next(&mut self, current_time: u64) -> Option<fn(SchedulerTaskContext)>
    {
        let Some(time) = self.next_task_time() else { return None };

        if time > current_time { return None };

        let entry = unsafe { self._tasks.pop_unchecked() };

        return Some(entry.task);
    }

    fn next_task_time(&self) -> Option<u64>
    {
        let Some(entry) = self._tasks.peek() else { return None };

        return Some(entry.cycles_after_init);
    }
}

pub(super) struct _TaskEntry
{
    pub priority: u8,
    pub cycles_after_init: u64,
    pub task: fn(SchedulerTaskContext),
}

impl PartialEq for _TaskEntry
{
    fn eq(&self, other: &Self) -> bool
    {
        self.priority == other.priority && self.cycles_after_init == other.cycles_after_init
    }
}

impl Eq for _TaskEntry { }

impl PartialOrd for _TaskEntry
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for _TaskEntry
{
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        match self.priority.cmp(&other.priority)
        {
            cmp::Ordering::Less => return cmp::Ordering::Less,
            cmp::Ordering::Greater => return cmp::Ordering::Greater,
            cmp::Ordering::Equal => (),
        }

        match self.cycles_after_init.cmp(&other.cycles_after_init)
        {
            cmp::Ordering::Less => return cmp::Ordering::Greater,
            cmp::Ordering::Greater => return cmp::Ordering::Less,
            cmp::Ordering::Equal => (),
        }

        return cmp::Ordering::Equal;
    }
}