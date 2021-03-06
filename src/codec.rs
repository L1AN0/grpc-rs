// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::call::MessageReader;
use crate::error::Result;

pub type DeserializeFn<T> = fn(MessageReader) -> Result<T>;
pub type SerializeFn<T> = fn(&T, &mut Vec<u8>);

/// Defines how to serialize and deserialize between the specialized type and byte slice.
pub struct Marshaller<T> {
    // Use function pointer here to simplify the signature.
    // Compiler will probably inline the function so performance
    // impact can be omitted.
    //
    // Using trait will require a trait object or generic, which will
    // either have performance impact or make signature complicated.
    //
    // const function is not stable yet (rust-lang/rust#24111), hence
    // make all fields public.
    /// The serialize function.
    pub ser: SerializeFn<T>,

    /// The deserialize function.
    pub de: DeserializeFn<T>,
}

#[cfg(feature = "protobuf-codec")]
pub mod pb_codec {
    use protobuf::{CodedInputStream, Message};

    use super::MessageReader;
    use crate::error::Result;

    #[inline]
    pub fn ser<T: Message>(t: &T, buf: &mut Vec<u8>) {
        t.write_to_vec(buf).unwrap()
    }

    #[inline]
    pub fn de<T: Message>(mut reader: MessageReader) -> Result<T> {
        let mut s = CodedInputStream::from_buffered_reader(&mut reader);
        let mut m = T::new();
        m.merge_from(&mut s)?;
        Ok(m)
    }
}

#[cfg(feature = "prost-codec")]
pub mod pr_codec {
    use bytes::buf::BufMut;
    use prost::Message;

    use super::MessageReader;
    use crate::error::Result;

    #[inline]
    pub fn ser<M: Message, B: BufMut>(msg: &M, buf: &mut B) {
        msg.encode(buf).expect("Writing message to buffer failed");
    }

    #[inline]
    pub fn de<M: Message + Default>(mut reader: MessageReader) -> Result<M> {
        use bytes::buf::Buf;
        reader.advance(0);
        M::decode(reader).map_err(Into::into)
    }
}

#[cfg(feature = "bincode-codec")]
pub mod bi_codec {
    use super::MessageReader;
    use crate::error::Result;
    use bytes::buf::{BufMut, BufMutExt};
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[inline]
    pub fn ser<M: Serialize, B: BufMut>(msg: &M, buf: &mut B) {
        bincode::serialize_into(buf.writer(), msg).expect("Writing message to buffer failed");
    }

    #[inline]
    pub fn de<M: DeserializeOwned>(mut reader: MessageReader) -> Result<M> {
        // use bytes::buf::Buf;
        // reader.advance(0);
        Ok(bincode::deserialize_from(reader).expect("Reading message from buffer failed"))
    }
}
