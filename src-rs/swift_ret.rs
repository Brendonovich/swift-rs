use crate::*;

pub unsafe trait SwiftRet {
    type SwiftType: SwiftRet;

    fn __retain(v: &Self::SwiftType);
}

unsafe impl SwiftRet for SRString {
    type SwiftType = SRString;

    fn __retain(v: &Self::SwiftType) {
        v.__retain();
    }
}

unsafe impl SwiftRet for String {
    type SwiftType = SRString;

    fn __retain(v: &Self::SwiftType) {
        v.__retain();
    }
}

unsafe impl SwiftRet for SRData {
    type SwiftType = SRData;

    fn __retain(v: &Self::SwiftType) {
        v.__retain();
    }
}

unsafe impl<T: SwiftRet> SwiftRet for SRObject<T> {
    type SwiftType = SRObject<T::SwiftType>;

    fn __retain(v: &Self::SwiftType) {
        v.__retain();
    }
}

unsafe impl<T: SwiftRet> SwiftRet for Option<T> {
    type SwiftType = Option<T::SwiftType>;

    fn __retain(v: &Self::SwiftType) {
        if let Some(v) = v {
            T::__retain(v);
        }
    }
}

unsafe impl<T: SwiftRet> SwiftRet for SRArray<T> {
    type SwiftType = SRArray<T::SwiftType>;

    fn __retain(v: &Self::SwiftType) {
        v.__retain();
    }
}

macro_rules! impl_primitive {
    ($t: ty) => {
        unsafe impl SwiftRet for $t {
            type SwiftType = $t;

            fn __retain(_: &Self::SwiftType) {}
        }
    };
}

impl_primitive!(Bool);
impl_primitive!(Int);
impl_primitive!(Int8);
impl_primitive!(Int16);
impl_primitive!(Int32);
impl_primitive!(Int64);
impl_primitive!(UInt);
impl_primitive!(UInt8);
impl_primitive!(UInt16);
impl_primitive!(UInt32);
impl_primitive!(UInt64);
impl_primitive!(Float);
impl_primitive!(Double);
