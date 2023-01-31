use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use crate::types::SRString;

#[must_use = "A SwiftRef MUST be sent over to the Swift side"]
#[repr(transparent)]
pub struct SwiftRef<T> {
    _pointer: NonNull<*const c_void>,
    _phantom: PhantomData<T>,
}

pub trait ToSwift<T> {
    /// Returns a [SwiftRef] to send to Swift ABI. This allows the Swift side function definitions
    /// to use the corresponding type directly, e.g.
    /// ```swift
    /// func getGreeting(name: SRString) {
    ///     // to use this signature, we need to invoke it from the rust side as
    ///     // unsafe { get_greeting(name.to_swift()) }
    /// }
    /// ```
    /// instead of
    /// ```swift
    /// func getGreeting(name: UnsafePointer<SRString>) {
    ///     // to use this signature, we need to invoke it from the rust side as
    ///     // unsafe { get_greeting(&name) }
    /// }
    /// ```
    ///
    /// # Safety
    /// This function is marked unsafe because it uses some specific kinds of pointer manipulation
    /// to allow a better developer API. Copying of this result, or manipulating it in other ways
    /// is very likely to lead to memory corruption.
    unsafe fn to_swift(&self) -> SwiftRef<T>;
}

macro_rules! impl_to_swift {
    ($name:ident$(<$generic:tt>)?) => {
        impl$(<$generic>)? crate::swift::ToSwift<$name$(<$generic>)?> for $name$(<$generic>)? {
            unsafe fn to_swift(&self) -> crate::swift::SwiftRef<$name$(<$generic>)?> {
                unsafe { std::mem::transmute_copy(self) }
            }
        }
    };
}
pub(crate) use impl_to_swift;

extern "C" {
    pub(crate) fn release_object(obj: *const c_void);
    pub(crate) fn allocate_string(data: *const u8, size: usize) -> SRString;
}
