// use core::convert::Infallible;

// pub trait DeserializeAsTag
// {
//     type Error;

//     fn serialize<W: CompoundDeserializerWrite>(self, writer: W)
//         -> Result<Result<W::Return, Self::Error>, W::Error>;
// }

// #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// pub enum Endian
// {
//     Little,
//     Big,
// }

// pub struct Deserializer<Value: DeserializeAsTag>
// {
//     _value: Value,
//     _endian: Endian,
// }

// impl<Value: DeserializeAsTag> Deserializer<Value>
// {
//     pub const fn new(value: Value, endian: Endian) -> Self
//     {
//         Self { _value: value, _endian: endian }
//     }
// }

// impl<Value: DeserializeAsTag> Deserialize for Deserializer<Value>
// {
//     type Word = u8;
//     type Error = Value::Error;

//     fn drain<
//         F: FnMut(u8) -> Result<(), E>,
//         E>(self, f: F) -> DeserializeResult<E, Value::Error>
//     {
//         match self._value.serialize(
//             DeserializerWriter { f, endian: self._endian })
//         {
//             Ok(Ok(_)) => DeserializeResult::Ok,
//             Ok(Err(error)) => DeserializeResult::DeserializeErr(error),
//             Err(error) => DeserializeResult::DrainErr(error),
//         }
//     }

//     fn drain_infallible<F: FnMut(u8)>(self, f: F)
//         -> DeserializeResult<Infallible, Value::Error>
//     {
//         let mut f = f;
//         let f = |byte| Ok::<(), Infallible>(f(byte));
//         match self._value.serialize(
//             DeserializerWriter { f, endian: self._endian })
//         {
//             Ok(Ok(_)) => DeserializeResult::Ok,
//             Ok(Err(error)) => DeserializeResult::DeserializeErr(error),
//         }
//     }
// }

// pub(super) struct DeserializerWriter<E, F: FnMut(u8) -> Result<(), E>>
// {
//     pub(super) f: F,
//     pub(super) endian: Endian,
// }

// pub trait Write
// {
//     type Error;
// }

// impl<E, F: FnMut(u8) -> Result<(), E>>
//     Write for DeserializerWriter<E, F>
// {
//     type Error = E;
// }

// pub trait WriteReturn: Write
// {
//     type Return;
// }

// impl<E, F: FnMut(u8) -> Result<(), E>>
//     WriteReturn for DeserializerWriter<E, F>
// {
//     type Return = Self;
// }