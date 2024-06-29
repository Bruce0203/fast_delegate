pub use delegare_derive::*;

pub trait Delegatable<T>: Sized {
    type Target;
    fn delegate_mut(&mut self) -> &mut Self::Target;
    fn delegate_ref(&self) -> &Self::Target;
    fn delegate_owned(self) -> Self::Target;
}

