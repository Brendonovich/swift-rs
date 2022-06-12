mod swift;
mod swift_arg;
mod swift_ret;
pub mod types;
pub use types::*;

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

            $(let $arg = $crate::SwiftArg::to_swift_type($arg);)*

            unsafe {
                let ret = $name($($arg),*);
                <$ret as $crate::SwiftRet>::retain(&ret);
                ret
            }
        }
    };
}
