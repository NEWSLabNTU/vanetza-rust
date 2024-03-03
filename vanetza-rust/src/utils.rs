use cxx::{private::UniquePtrTarget, UniquePtr};

pub(crate) trait ToCxxUniquePtr<T>
where
    T: UniquePtrTarget,
{
    fn to_cxx_unique_ptr(&self) -> UniquePtr<T>;
}

pub(crate) trait ToCxxPod<T> {
    fn to_cxx_pod(&self) -> T;
}

pub(crate) trait FromCxxRef<T> {
    fn from_cxx_ref(src: &T) -> Self;
}
