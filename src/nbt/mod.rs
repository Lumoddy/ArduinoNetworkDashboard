
mod tag;
mod binary;
mod reader;
mod writer;

pub use tag::*;
pub use binary::*;
pub use reader::*;
pub use writer::*;

// Spec according to:
// - https://github.com/acfoltzer/nbt/blob/master/NBT-spec.txt
// - https://minecraft.wiki/w/NBT_format