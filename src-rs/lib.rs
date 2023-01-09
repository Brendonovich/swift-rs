mod autorelease;
mod objc;
mod swift_arg;
mod swift_ret;
mod types;

pub use autorelease::*;
pub use swift_arg::*;
pub use swift_ret::*;
pub use swift_rs_macros::*;
pub use types::*;

#[cfg(feature = "build")]
pub mod build;

#[macro_export]
macro_rules! swift  {
    ($vis:vis unsafe fn $name:ident($($arg:ident: $t:ty),*) $(-> $ret:ty)?) => {
        $vis unsafe fn $name($($arg: $t),*) $( -> <$ret as $crate::SwiftRet>::SwiftType)? {
            $crate::swift!(impl $name($($arg: $t),*) $(-> $ret)?)
        }
    };
    (impl $name:ident($($arg:ident: $t:ty),*) $(-> $ret:ty)?) => {{
        extern "C" {
            fn $name($($arg: <$t as $crate::SwiftArg>::SwiftType),*) $(-> <$ret as $crate::SwiftRet>::SwiftType)?;
        }

        $(
            let pool = unsafe { $crate::objc_autoreleasePoolPush() };
            let _: $ret;
        )?

        $(
            let $arg = <$t as $crate::SwiftArg>::to_swift_rs_type($arg);
            let $arg = <$t as $crate::SwiftArg>::as_swift_type(&$arg);
        )*

        unsafe {
            let ret = $name($($arg),*);
            $(
                <$ret as $crate::SwiftRet>::__retain(&ret);
                $crate::objc_autoreleasePoolPop(pool);
            )?
            ret
        }
    }};
}
