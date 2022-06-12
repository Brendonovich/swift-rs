use crate::{SRObject, SRString};

pub unsafe trait SwiftRet {
    type SwiftType;

    fn retain(v: &Self::SwiftType);
}

unsafe impl SwiftRet for SRString {
    type SwiftType = SRString;

    fn retain(v: &Self::SwiftType) {
        v.retain();
    }
}

unsafe impl SwiftRet for String {
    type SwiftType = SRString;

    fn retain(v: &Self::SwiftType) {
        v.retain()
    }
}

macro_rules! impl_primitive {
    ($t: ty) => {
        unsafe impl SwiftRet for $t {
            type SwiftType = $t;

            fn retain(_: &Self::SwiftType) {}
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
