use crate::{swift::SwiftObject, *};
use std::ffi::c_void;

pub trait SwiftRet {}

macro_rules! primitive_impl {
    ($($t:ty),+) => {
        $(impl SwiftRet for $t {})+
    };
}

primitive_impl!(
    Bool,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    *const c_void,
    *mut c_void,
    *const u8,
    ()
);

impl<T: SwiftObject> SwiftRet for T {}
