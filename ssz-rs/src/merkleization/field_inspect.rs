//! A trait that allows you to inspect the fields of structs.
//!
//! See the docs on [`FieldsInspect`] for more.
#![allow(dead_code)]

use crate::SszReflect;
use core::{iter::FusedIterator, marker::PhantomData, panic::UnwindSafe};

// FIXME: We're forced to use a second trait because `*mut Self` cannot be the `self` pointer
// without `arbitrary_self_types`. When it is stabilized, remove this trait.
#[doc(hidden)]
pub trait FieldsInspectImpl {
    fn struct_name() -> &'static str;
    fn fields_count() -> u32;
    fn field_name(n: u32) -> &'static str;
    fn field(&self, n: u32) -> &dyn SszReflect;
    /// # Safety
    ///
    /// You should call this only with `this` pointing to a valid `FieldsInspectImpl`-implementing
    /// type (this is not a reference because this runs into issues with Stacked Borrows for
    /// `FieldsIterMut`). You should use the resulting reference only for the lifetime of `this`,
    /// and not create an overlapping reference (references from the same `n`) during this region.
    unsafe fn field_mut(this: *mut (), n: u32) -> &'static mut dyn SszReflect;
}

impl<'a, T: ?Sized + FieldsInspectImpl + 'a> FieldsInspectImpl for &'a mut T {
    fn struct_name() -> &'static str {
        T::struct_name()
    }
    fn fields_count() -> u32 {
        T::fields_count()
    }
    fn field_name(n: u32) -> &'static str {
        T::field_name(n)
    }
    fn field(&self, n: u32) -> &dyn SszReflect {
        T::field(*self, n)
    }
    unsafe fn field_mut(this: *mut (), n: u32) -> &'static mut dyn SszReflect {
        // SAFETY: Precondition.
        unsafe { T::field_mut((*this.cast::<Self>()) as *mut T as *mut (), n) }
    }
}

#[doc(hidden)]
pub struct FieldsIterMutVtable {
    field_name: fn(u32) -> &'static str,
    field_mut: unsafe fn(*mut (), n: u32) -> &'static mut dyn SszReflect,
}

/// A trait that allows iterating over over struct's fields, getting their name and a mutable/shared
/// reference to them.
///
/// [You need to derive this trait](derive@FieldsInspect) (actually, it the derive creates an impl
/// for a hidden trait this trait has a blanket implementation to).
///
/// # Examples
///
/// Printing the values of all field whose name starts with "a" and are strings:
/// ```
/// use fields_iter::{FieldsInspect, FieldsIter};
///
/// fn print_starts_with_a(v: &dyn FieldsInspect) {
///     for (name, value) in FieldsIter::new(v) {
///         if !name.starts_with('a') { continue };
///         let Some(value) = value.downcast_ref::<String>() else { continue };
///         println!("{name}={value}");
///     }
/// }
/// ```
///
/// Adding one to the field `add_here`:
/// ```
/// use fields_iter::{FieldsInspect, FieldsIterMut};
///
/// # #[derive(FieldsInspect)]
/// # struct Type { add_here: i32 }
/// # let mut original = Type { add_here: 0 };
/// let v: &mut dyn FieldsInspect;
/// # let v: &mut dyn FieldsInspect = &mut original;
/// let field = FieldsIterMut::new(v)
///     .find(|&(name, _)| name == "add_here")
///     .expect("no `add_here` field")
///     .1
///     .downcast_mut::<i32>()
///     .expect("field `add_here` is not of type `i32`");
/// *field += 1;
/// # assert_eq!(original.add_here, 1);
/// ```
pub trait FieldsInspect {
    /// The struct name.
    ///
    /// This takes `&self` to make `FieldsIter` object safe.
    fn struct_name(&self) -> &'static str;

    /// The numbers of fields.
    ///
    /// This allows you to iterate over the fields without allocating a `Box` and in `no_std` mode.
    ///
    /// This takes `&self` to make `FieldsIter` object safe.
    ///
    /// # Example
    ///
    /// ```
    /// # use fields_iter::FieldsInspect;
    /// #[derive(FieldsInspect)]
    /// struct HasFieldsInspect {
    ///     a: i32,
    ///     b: String,
    /// }
    ///
    /// let v = HasFieldsInspect { a: 0, b: String::new() };
    /// assert_eq!(v.fields_count(), 2);
    /// ```
    ///
    /// This takes `&self` to make `FieldsIter` object safe.
    fn fields_count(&self) -> u32;

    /// The name of the nth field.
    ///
    /// Named fields return their name; tuple fields return their index (e.g. "1", "2").
    ///
    /// This allows you to iterate over the fields without allocating a `Box` and in `no_std` mode.
    ///
    /// This takes `&self` to make `FieldsIter` object safe.
    ///
    /// # Example
    ///
    /// ```
    /// # use fields_iter::FieldsInspect;
    /// #[derive(FieldsInspect)]
    /// struct HasFieldsInspect {
    ///     a: i32,
    ///     b: String,
    /// }
    ///
    /// let v = HasFieldsInspect { a: 0, b: String::new() };
    /// assert_eq!(v.field_name(0), "a");
    /// assert_eq!(v.field_name(1), "b");
    /// ```
    ///
    /// # Panics
    ///
    /// This panics if given an out of bounds field index.
    fn field_name(&self, n: u32) -> &'static str;

    /// The value of the nth field.
    ///
    /// This allows you to iterate over the fields without allocating a `Box` and in `no_std` mode.
    ///
    /// # Example
    ///
    /// ```
    /// # use fields_iter::FieldsInspect;
    /// #[derive(FieldsInspect)]
    /// struct HasFieldsInspect {
    ///     a: i32,
    ///     b: String,
    /// }
    ///
    /// let v = HasFieldsInspect { a: 0, b: String::new() };
    /// assert!(std::ptr::eq(v.field(0), &v.a));
    /// assert!(std::ptr::eq(v.field(1), &v.b));
    /// ```
    ///
    /// # Panics
    ///
    /// This panics if given an out of bounds field index.
    fn field(&self, n: u32) -> &dyn SszReflect;

    /// The value of the nth field.
    ///
    /// This allows you to iterate over the fields without allocating a `Box` and in `no_std` mode.
    ///
    /// # Example
    ///
    /// ```
    /// # use fields_iter::FieldsInspect;
    /// #[derive(FieldsInspect)]
    /// struct HasFieldsInspect {
    ///     a: i32,
    ///     b: String,
    /// }
    ///
    /// let mut v = HasFieldsInspect { a: 0, b: String::new() };
    /// *v.field_mut(0).downcast_mut::<i32>().unwrap() += 5;
    /// assert_eq!(v.a, 5);
    /// ```
    ///
    /// # Panics
    ///
    /// This panics if given an out of bounds field index.
    fn field_mut(&mut self, n: u32) -> &mut dyn SszReflect;

    #[doc(hidden)]
    fn __fields_mut_iter_vtable(&self) -> &'static FieldsIterMutVtable;
}

impl<T: ?Sized + FieldsInspectImpl> FieldsInspect for T {
    fn struct_name(&self) -> &'static str {
        <T as FieldsInspectImpl>::struct_name()
    }
    fn fields_count(&self) -> u32 {
        <T as FieldsInspectImpl>::fields_count()
    }
    fn field_name(&self, n: u32) -> &'static str {
        <T as FieldsInspectImpl>::field_name(n)
    }
    fn field(&self, n: u32) -> &dyn SszReflect {
        <T as FieldsInspectImpl>::field(self, n)
    }
    fn field_mut(&mut self, n: u32) -> &mut dyn SszReflect {
        // SAFETY: We derive the pointer from reference and return a reference with the same
        // lifetime.
        unsafe { <T as FieldsInspectImpl>::field_mut(self as *mut Self as *mut (), n) }
    }
    #[doc(hidden)]
    fn __fields_mut_iter_vtable(&self) -> &'static FieldsIterMutVtable {
        &FieldsIterMutVtable {
            field_name: <Self as FieldsInspectImpl>::field_name,
            field_mut: <Self as FieldsInspectImpl>::field_mut,
        }
    }
}

/// An iterator over the names and shared references to a type implementing `FieldsInspect`.
///
/// # Example
///
/// ```
/// # use ssz_rs::SszReflect;
/// # use fields_iter::{FieldsInspect, FieldsIter};
/// fn find_field<'a>(v: &'a dyn FieldsInspect, name: &str) -> Option<&'a dyn SszReflect> {
///     FieldsIter::new(v).find_map(|(n, v)| (n == name).then_some(v))
/// }
///
/// #[derive(FieldsInspect)]
/// struct HasFieldsInspect {
///     a: i32,
///     b: String,
/// }
/// let v = HasFieldsInspect { a: 0, b: String::new() };
/// assert!(std::ptr::eq(&v.b, find_field(&v, "b").unwrap().as_any().downcast_ref::<String>().unwrap()));
/// ```
pub struct FieldsIter<'a, T: ?Sized = dyn FieldsInspect> {
    fields_count: u32,
    next_field_idx: u32,
    value: &'a T,
}

impl<'a, T: ?Sized + FieldsInspect> FieldsIter<'a, T> {
    /// Creates a new `FieldsIter`.
    pub fn new(v: &'a T) -> Self {
        Self { fields_count: v.fields_count(), next_field_idx: 0, value: v }
    }
}

impl<'a, T: ?Sized + FieldsInspect> Iterator for FieldsIter<'a, T> {
    type Item = (&'static str, &'a dyn SszReflect);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_field_idx >= self.fields_count {
            return None
        }
        let name = self.value.field_name(self.next_field_idx);
        let value = self.value.field(self.next_field_idx);
        self.next_field_idx += 1;
        Some((name, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let result = self.len();
        (result, Some(result))
    }
}

impl<'a, T: ?Sized + FieldsInspect> ExactSizeIterator for FieldsIter<'a, T> {
    fn len(&self) -> usize {
        (self.fields_count - self.next_field_idx) as usize
    }
}

impl<'a, T: ?Sized + FieldsInspect> FusedIterator for FieldsIter<'a, T> {}

impl<'a, T: ?Sized + FieldsInspect> DoubleEndedIterator for FieldsIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.fields_count == self.next_field_idx {
            return None
        }
        self.fields_count -= 1;
        let name = self.value.field_name(self.fields_count);
        let value = self.value.field(self.fields_count);
        Some((name, value))
    }
}

/// An iterator over the names and mutable references to a type implementing `FieldsInspect`.
///
/// # Example
///
/// ```
/// # use ssz_rs::SszReflect;
/// # use fields_iter::{FieldsInspect, FieldsIterMut};
/// fn find_field<'a>(v: &'a mut dyn FieldsInspect, name: &str) -> Option<&'a mut dyn SszReflect> {
///     FieldsIterMut::new(v).find_map(|(n, v)| (n == name).then_some(v))
/// }
///
/// #[derive(FieldsInspect)]
/// struct HasFieldsInspect {
///     a: i32,
///     b: String,
/// }
/// let mut v = HasFieldsInspect { a: 0, b: String::new() };
/// *find_field(&mut v, "a").unwrap().as_any().downcast_mut::<i32>().unwrap() = 123;
/// assert_eq!(v.a, 123);
/// ```
pub struct FieldsIterMut<'a, T: ?Sized = dyn FieldsInspect> {
    fields_count: u32,
    next_field_idx: u32,
    value: *mut (),
    vtable: &'static FieldsIterMutVtable,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: ?Sized + FieldsInspect> FieldsIterMut<'a, T> {
    /// Creates a new `FieldsIter`.
    pub fn new(v: &'a mut T) -> Self {
        Self {
            fields_count: v.fields_count(),
            next_field_idx: 0,
            value: v as *mut T as *mut (),
            vtable: v.__fields_mut_iter_vtable(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T: ?Sized + FieldsInspect> Iterator for FieldsIterMut<'a, T> {
    type Item = (&'static str, &'a mut dyn SszReflect);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_field_idx >= self.fields_count {
            return None
        }
        let name = (self.vtable.field_name)(self.next_field_idx);
        // The pointer is created from reference by `new()`, we return a reference with the same
        // lifetime, and we only borrow disjoint fields.
        let value = unsafe { (self.vtable.field_mut)(self.value, self.next_field_idx) };
        self.next_field_idx += 1;
        Some((name, value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let result = self.len();
        (result, Some(result))
    }
}

impl<'a, T: ?Sized + FieldsInspect> ExactSizeIterator for FieldsIterMut<'a, T> {
    fn len(&self) -> usize {
        (self.fields_count - self.next_field_idx) as usize
    }
}

impl<'a, T: ?Sized + FieldsInspect> FusedIterator for FieldsIterMut<'a, T> {}

impl<'a, T: ?Sized + FieldsInspect> DoubleEndedIterator for FieldsIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.fields_count == self.next_field_idx {
            return None
        }
        self.fields_count -= 1;
        let name = (self.vtable.field_name)(self.fields_count);
        // The pointer is created from reference by `new()`, we return a reference with the same
        // lifetime, and we only borrow disjoint fields.
        let value = unsafe { (self.vtable.field_mut)(self.value, self.fields_count) };
        Some((name, value))
    }
}

// SAFETY: We use the pointer, our only non-`Send`/`Sync` data, in a uniquely owned manner.
unsafe impl<'a, T: ?Sized + FieldsInspect + Send> Send for FieldsIterMut<'a, T> {}
unsafe impl<'a, T: ?Sized + FieldsInspect + Sync> Sync for FieldsIterMut<'a, T> {}
impl<'a, T: ?Sized + FieldsInspect + UnwindSafe> UnwindSafe for FieldsIterMut<'a, T> {}

#[cold]
#[doc(hidden)]
#[track_caller]
pub fn field_out_of_bounds(struct_name: &str, field: u32) -> ! {
    panic!("field index {field} is out of bounds for struct `{struct_name}`")
}

#[doc(hidden)]
pub use core::ptr::addr_of_mut;
