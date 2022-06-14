mod objc;
mod swift_arg;
mod swift_ret;
pub mod types;
pub use types::*;

pub use swift_rs_macros::*;

pub use swift_arg::SwiftArg;
pub use swift_ret::SwiftRet;

#[cfg(feature = "build")]
pub mod build;

#[macro_export]
macro_rules! swift_fn_inner {
    ($name: ident($($arg: ident: $t: ty),*)) => {{
        extern "C" {
            fn $name($($arg: <$t as $crate::SwiftArg>::SwiftType),*);
        }

        $(let $arg = $crate::SwiftArg::to_swift_rs_type($arg);)*

        unsafe {
            $name($($arg),*);
        }
    }};
    ($name: ident($($arg: ident: $t: ty),*) -> $ret: ty) => {{
        extern "C" {
            fn $name($($arg: <$t as $crate::SwiftArg>::SwiftType),*) -> <$ret as $crate::SwiftRet>::SwiftType;
            fn objc_autoreleasePoolPush() -> *mut std::os::raw::c_void;
            fn objc_autoreleasePoolPop(context: *mut std::os::raw::c_void);
        }

        let pool = unsafe { objc_autoreleasePoolPush() };

        $(
            let $arg = <$t as $crate::SwiftArg>::to_swift_rs_type($arg);
            let $arg = <$t as $crate::SwiftArg>::as_swift_type(&$arg);
        )*


        unsafe {
            let ret = $name($($arg),*);
            <$ret as $crate::SwiftRet>::__retain(&ret);
            objc_autoreleasePoolPop(pool);
            ret
        }
    }};
}

#[macro_export]
macro_rules! swift_fn {
    ($name: ident($($arg: ident: $t: ty),*)) => {
        fn $name($($arg: $t),*) {
            $crate::swift_fn_inner!($name($($arg: $t),*))
        }
    };
    ($name: ident($($arg: ident: $t: ty),*) -> $ret: ty) => {
        fn $name($($arg: $t),*) -> <$ret as $crate::SwiftRet>::SwiftType {
            $crate::swift_fn_inner!($name($($arg: $t),*) -> $ret)
        }
    };
}

#[macro_export]
macro_rules! pub_swift_fn {
    ($name: ident($($arg: ident: $t: ty),*)) => {
        pub fn $name($($arg: $t),*) {
            $crate::swift_fn_inner!($name($($arg: $t),*))
        }
    };
    ($name: ident($($arg: ident: $t: ty),*) -> $ret: ty) => {
        pub fn $name($($arg: $t),*) -> <$ret as $crate::SwiftRet>::SwiftType {
            $crate::swift_fn_inner!($name($($arg: $t),*) -> $ret)
        }
    };
}
