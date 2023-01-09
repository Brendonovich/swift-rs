use std::ffi::c_void;

use crate::*;

pub unsafe trait SwiftArg {
    type SwiftRsType;
    type SwiftType;

    fn to_swift_rs_type(self) -> Self::SwiftRsType;
    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType;
}

macro_rules! impl_primitive {
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

unsafe impl SwiftArg for SRString {
    type SwiftRsType = SRString;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        rs_type.0 .0 .0.as_ptr() as Self::SwiftType
    }
}

unsafe impl SwiftArg for SRData {
    type SwiftRsType = SRData;
    type SwiftType = *const c_void;

    fn to_swift_rs_type(self) -> Self::SwiftRsType {
        self
    }

    fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
        rs_type.0 .0.as_ptr() as Self::SwiftType
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

// unsafe impl SwiftArg for String {
//     type SwiftRsType = SRString;
//     type SwiftType = *const c_void;

//     fn to_swift_rs_type(self) -> Self::SwiftRsType {
//         self.as_str().to_swift_rs_type()
//     }

//     fn as_swift_type(rs_type: &Self::SwiftRsType) -> Self::SwiftType {
//         Self::SwiftRsType::as_swift_type(rs_type)
//     }
// }

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
