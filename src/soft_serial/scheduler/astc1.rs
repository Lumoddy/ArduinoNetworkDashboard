use core::cmp::Ordering;

use arduino_hal::pac::{tc1::tccr1b, TC1};
use super::*;

pub trait Prescaler { const CS: tccr1b::CS1_A; }

struct NoPrescaler { _private: () }
struct PrescaleBy8 { _private: () }
struct PrescaleBy64 { _private: () }
struct PrescaleBy256 { _private: () }
struct PrescaleBy1024 { _private: () }

impl Prescaler for NoPrescaler
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::DIRECT;
}
impl Prescaler for PrescaleBy8
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_8;
}
impl Prescaler for PrescaleBy64
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_64;
}
impl Prescaler for PrescaleBy256
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_256;
}
impl Prescaler for PrescaleBy1024
{
    const CS: tccr1b::CS1_A = tccr1b::CS1_A::PRESCALE_1024;
}

trait _SchedulerAllocationOps
{

}

pub struct SchedulerAllocation<const TASK_CAPACITY: usize>
{
    _tasks: heapless::BinaryHeap<
        _TaskEntry,
        heapless::binary_heap::Max,
        TASK_CAPACITY>,
}

impl<const TASK_CAPACITY: usize> SchedulerAllocation<TASK_CAPACITY>
{
    pub const fn new() -> Self { Self { _tasks: heapless::BinaryHeap::new() } }
}

impl<const TASK_CAPACITY: usize> Default for SchedulerAllocation<TASK_CAPACITY>
{
    fn default() -> Self { Self::new() }
}

pub struct Scheduler
{
    _tc1: TC1,
    _tasks: heapless::BinaryHeap<
        _TaskEntry<Task, Priority>,
        heapless::binary_heap::Min,
        CAPACITY>,
    _cycle_count_at_last_overflow: u64,
    _clock: PhantomData<Clock>,
    _prescaler: PhantomData<Prescaler>,
}

struct _TaskEntry
{
    pub priority: u8,
    pub time: SchedulerCounterInt,
    pub task: SchedulerTask,
}

impl PartialEq for _TaskEntry
{
    fn eq(&self, other: &Self) -> bool
    {
        self.priority == other.priority && self.time == other.time
    }
}

impl Eq for _TaskEntry { }

impl PartialOrd for _TaskEntry
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for _TaskEntry
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.priority.cmp(&other.priority)
        {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }

        match self.time.cmp(&other.time)
        {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }

        return Ordering::Equal;
    }
}