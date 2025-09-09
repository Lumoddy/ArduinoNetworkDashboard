
mod tag;
mod binary;

pub use tag::*;
pub use binary::*;
pub mod writer;

// Spec according to:
// - https://github.com/acfoltzer/nbt/blob/master/NBT-spec.txt
// - https://minecraft.wiki/w/NBT_format