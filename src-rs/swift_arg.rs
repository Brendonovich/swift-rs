use crate::*;

pub unsafe trait SwiftArg {
    type SwiftType;

    fn to_swift_type(self) -> Self::SwiftType;
}

unsafe impl SwiftArg for String {
    type SwiftType = SRString;

    fn to_swift_type(self) -> Self::SwiftType {
        SRString::from(self.as_ref())
    }
}

unsafe impl SwiftArg for &str {
    type SwiftType = SRString;

    fn to_swift_type(self) -> Self::SwiftType {
        SRString::from(self)
    }
}

macro_rules! self_impl {
    ($t: ty) => {
        unsafe impl SwiftArg for $t {
            type SwiftType = $t;

            fn to_swift_type(self) -> Self::SwiftType {
                self
            }
        }
    };
}

self_impl!(bool);
self_impl!(isize);
self_impl!(i8);
self_impl!(i16);
self_impl!(i32);
self_impl!(i64);
self_impl!(usize);
self_impl!(u8);
self_impl!(u16);
self_impl!(u32);
self_impl!(u64);
self_impl!(f32);
self_impl!(f64);

self_impl!(SRData);
self_impl!(SRString);