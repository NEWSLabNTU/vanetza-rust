use std::{marker::PhantomData, ops::Deref, pin::Pin};

use cxx::{private::UniquePtrTarget, UniquePtr};

pub(crate) trait IntoCxxUniquePtr<T>
where
    T: UniquePtrTarget,
{
    fn into_cxx_unique_ptr(self) -> UniquePtr<T>;
}

pub(crate) trait ToCxxUniquePtr<T>
where
    T: UniquePtrTarget,
{
    fn to_cxx_unique_ptr(&self) -> UniquePtr<T>;
}

pub(crate) trait FromCxxUniquePtr<T>
where
    T: UniquePtrTarget,
{
    fn from_cxx_unique_ptr(ptr: UniquePtr<T>) -> Self;
}

pub(crate) trait ToCxxPod<T> {
    fn to_cxx_pod(&self) -> T;
}

pub(crate) trait ToCxxRefMut<T>
where
    T: UniquePtrTarget,
{
    fn to_cxx_ref_mut(&mut self) -> CxxRefMut<'_, T>;
}

pub(crate) trait FromCxxRef<T> {
    fn from_cxx_ref(ref_: &T) -> Self;
}

pub(crate) struct CxxRefMut<'a, T>
where
    T: UniquePtrTarget,
{
    ptr: UniquePtr<T>,
    _phatom: PhantomData<&'a ()>,
}

impl<'a, T> CxxRefMut<'a, T>
where
    T: UniquePtrTarget,
{
    pub unsafe fn new(ptr: UniquePtr<T>) -> Self {
        Self {
            ptr,
            _phatom: PhantomData,
        }
    }

    pub fn pin_mut(&mut self) -> Pin<&mut T> {
        self.ptr.pin_mut()
    }
}

impl<'a, T> Deref for CxxRefMut<'a, T>
where
    T: UniquePtrTarget,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
