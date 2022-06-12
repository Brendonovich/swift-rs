use crate::{SRArray, SRData, SRObject, SRString};

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

impl_primitive!(bool);
impl_primitive!(isize);
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(usize);
impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(f32);
impl_primitive!(f64);
