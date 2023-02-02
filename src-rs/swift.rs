use std::ffi::c_void;

use crate::*;

#[must_use = "A Ref MUST be sent over to the Swift side"]
#[repr(transparent)]
pub struct SwiftRef<'a, T: SwiftObject>(&'a SRObjectImpl<T::Shape>);

pub trait SwiftObject {
    type Shape;

    /// Gets a reference to the `SRObject` at the root of a `SwiftObject`
    fn get_object(&self) -> &SRObject<Self::Shape>;

    /// Creates a [`SwiftRef`] for an object which can be used when calling a Swift function.
    /// This function should never be called manually,
    /// instead you should rely on the [`swift!`] macro to call it for you.
    ///
    /// # Safety
    /// This function converts the [`NonNull`](std::ptr::NonNull)
    /// inside an [`SRObject`] into a reference,
    /// implicitly assuming that the pointer is still valid.
    /// The inner pointer is private,
    /// and the returned [`SwiftRef`] is bound to the lifetime of the original [`SRObject`],
    /// so if you use `swift-rs` as normal this function should be safe.
    unsafe fn swift_ref(&self) -> SwiftRef<Self>
    where
        Self: Sized,
    {
        SwiftRef(self.get_object().0.as_ref())
    }
}

/// Declares a function defined in a swift library.
/// Use this macro as if the contents were going directly
/// into an `extern "C"` block.
///
///
/// # Examples
///
/// ```ignore
/// use swift_rs::*;
///
/// swift!(fn echo(string: &SRString) -> SRString);
///
/// fn main() {
///     let string: SRString = "test".into();
///     let result = unsafe { echo(&string) };
///
///     assert_eq!(result.as_str(), string.as_str())
/// }
///
/// ```
///
/// # Details
///
/// Internally, this macro creates an `unsafe` function containing
/// an `extern "C"` block declaring the actual swift function,
/// conversion of arguments implementing [`SwiftObject`] to [`SwiftRef`],
/// and finally a call to the swift function.
#[macro_export]
macro_rules! swift {
    ($vis:vis fn $name:ident($($arg:ident: $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis unsafe fn $name($($arg: $arg_ty),*) $(-> $ret)? {
            extern "C" {
                fn $name($($arg: <$arg_ty as $crate::SwiftArg>::ArgType),*) $(-> $ret)?;
            }

            $(let $arg = $crate::SwiftArg::as_arg(&$arg);)*

            $name($($arg),*)
        }
    };
}

swift!(pub(crate) fn release_object(obj: *const c_void));
swift!(pub(crate) fn allocate_string(data: *const u8, size: UInt) -> SRString);
