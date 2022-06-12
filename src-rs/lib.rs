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
macro_rules! swift_fn {
    ($name: ident($($arg: ident: $t: ty),*)) => {
        fn $name($($arg: $t),*) {
            extern "C" {
                fn $name($($arg: <$t as $crate::SwiftArg>::SwiftType),*);
            }

            $(let $arg = $crate::SwiftArg::to_swift_type($arg);)*

            unsafe {
                $name($($arg),*);
            }
        }
    };
    ($name: ident($($arg: ident: $t: ty),*) -> $ret: ty) => {
        fn $name($($arg: $t),*) -> <$ret as $crate::SwiftRet>::SwiftType {
            extern "C" {
                fn $name($($arg: <$t as $crate::SwiftArg>::SwiftType),*) -> <$ret as $crate::SwiftRet>::SwiftType;
            }
            extern "C" {
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
        }
    };
}
