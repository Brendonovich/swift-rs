use std::ffi::c_void;

use crate::*;

pub unsafe trait SwiftArg {
    type SwiftRsType;
    type SwiftType;

    fn to_swift_rs_type(self) -> Self::SwiftRsType;
    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType;
}

macro_rules! primitive_impl {
    ($t: ty) => {
        unsafe impl SwiftArg for $t {
            type SwiftRsType = $t;
            type SwiftType = $t;

            fn to_swift_rs_type(self) -> Self::SwiftRsType {
                (self)
            }

            fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
                *rs_type
            }
        }
    };
}

primitive_impl!(bool);
primitive_impl!(isize);
primitive_impl!(i8);
primitive_impl!(i16);
primitive_impl!(i32);
primitive_impl!(i64);
primitive_impl!(usize);
primitive_impl!(u8);
primitive_impl!(u16);
primitive_impl!(u32);
primitive_impl!(u64);
primitive_impl!(f32);
primitive_impl!(f64);

unsafe impl SwiftArg for SRString {
    type SwiftRsType = SRString;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        rs_type.0 .0 .0.as_ptr() as *const _ as *const c_void
    }
}

unsafe impl SwiftArg for SRData {
    type SwiftRsType = SRData;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        rs_type.0 .0.as_ptr() as *const c_void
    }
}

unsafe impl SwiftArg for &str {
    type SwiftRsType = SRString;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        SRString::from(self)
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        Self::SwiftRsType::as_swift_type(rs_type)
    }
}

unsafe impl SwiftArg for String {
    type SwiftRsType = SRString;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self.as_str().to_swift_rs_type()
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        Self::SwiftRsType::as_swift_type(rs_type)
    }
}

unsafe impl<T: SwiftArg> SwiftArg for *const T {
    type SwiftRsType = *const c_void;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self as *const _ as *const _
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        *rs_type
    }
}

unsafe impl<T: SwiftArg> SwiftArg for *mut T {
    type SwiftRsType = *const c_void;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self as *mut _ as *mut _
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        *rs_type
    }
}

unsafe impl SwiftArg for *const c_void {
    type SwiftRsType = *const c_void;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        *rs_type
    }
}

unsafe impl SwiftArg for *mut c_void {
    type SwiftRsType = *mut c_void;
    type SwiftType = *mut c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        *rs_type
    }
}