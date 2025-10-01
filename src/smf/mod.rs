//! Small Message Format
//!
//! Syntax:
//! ```
//! SMF = $(Block | Sequence)+
//!
//! Sequence = 1 Block Sequence | 0
//!
//! Block = u8 $(Block)?
//! ```

mod visitor;
mod tracer;
mod reader;
mod writer;

pub use visitor::*;
pub use tracer::*;
pub use reader::*;
pub use writer::*;