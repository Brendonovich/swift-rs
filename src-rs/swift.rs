use std::{cmp::min, collections::BTreeMap, ffi::c_void};

use crate::*;

#[must_use = "A Ref MUST be sent over to the Swift side"]
#[repr(transparent)]
pub struct SwiftRef<'a, T: SwiftObject>(&'a SRObjectImpl<T::Shape>);

impl<'a, T: SwiftObject> SwiftRef<'a, T> {
    pub(crate) fn as_ptr(&self) -> *const c_void {
        self.0 as *const _ as *const c_void
    }
}

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

swift!(pub(crate) fn retain_object(obj: *const c_void));
swift!(pub(crate) fn release_object(obj: *const c_void));
swift!(pub(crate) fn allocate_string(data: *const u8, size: UInt) -> SRString);

#[derive(Clone, Copy, Debug)]
enum ValueArity {
    Reference,
    Value,
}

pub unsafe fn balance_ptrs(args: Vec<(*const c_void, bool)>, ret: Vec<(*const c_void, bool)>) {
    fn collect_references(
        v: Vec<(*const c_void, bool)>,
    ) -> BTreeMap<*const c_void, Vec<ValueArity>> {
        v.into_iter().fold(
            BTreeMap::<_, Vec<ValueArity>>::new(),
            |mut map, (ptr, is_ref)| {
                map.entry(ptr).or_default().push(if is_ref {
                    ValueArity::Reference
                } else {
                    ValueArity::Value
                });
                map
            },
        )
    }

    let mut args = collect_references(args);
    let mut ret = collect_references(ret);

    let both_counts = args
        .clone()
        .into_iter()
        .flat_map(|(arg, values)| {
            ret.remove(&arg).map(|ret| {
                args.remove(&arg);

                let ret_values = ret
                    .iter()
                    .filter(|v| matches!(v, ValueArity::Value))
                    .count() as isize;

                let arg_references = values
                    .iter()
                    .filter(|v| matches!(v, ValueArity::Reference))
                    .count() as isize;

                let ref_in_value_out_retains = min(ret_values, arg_references);

                (arg, ref_in_value_out_retains)
            })
        })
        .collect::<Vec<_>>();

    let arg_counts = args.into_iter().map(|(ptr, values)| {
        let count = values
            .into_iter()
            .filter(|v| matches!(v, ValueArity::Value))
            .count() as isize;
        (ptr, count)
    });

    let ret_counts = ret
        .into_iter()
        .map(|(ptr, values)| {
            let count = values
                .into_iter()
                .filter(|v| matches!(v, ValueArity::Value))
                .count() as isize;
            (ptr, count)
        })
        .collect::<Vec<_>>();

    both_counts
        .into_iter()
        .chain(arg_counts)
        .chain(ret_counts)
        .for_each(|(ptr, count)| match count {
            0 => {}
            n if n > 0 => {
                for _ in 0..n {
                    retain_object(ptr)
                }
            }
            n => {
                for _ in n..0 {
                    release_object(ptr)
                }
            }
        });
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
    ($vis:vis fn $name:ident $(<$($lt:lifetime),+>)? ($($arg:ident: $arg_ty:ty),*) $(-> $ret:ty)?) => {
        $vis unsafe fn $name $(<$($lt),*>)? ($($arg: $arg_ty),+) $(-> $ret)? {
            extern "C" {
                fn $name $(<$($lt),*>)? ($($arg: <$arg_ty as $crate::SwiftArg>::ArgType),*) $(-> $ret)?;
            }

            let arg_ptrs = vec![$($crate::SwiftArg::collect_ptrs(&$arg, false)),*]
                .into_iter()
                .flatten()
                .collect();

            let res = {
                $(let $arg = $crate::SwiftArg::as_arg(&$arg);)*

                $name($($arg),*)
            };

            let res_ptrs = $crate::SwiftArg::collect_ptrs(&res, false);

            $crate::balance_ptrs(arg_ptrs, res_ptrs);

            res
        }
    };
}
