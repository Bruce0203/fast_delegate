pub use fast_delegate_derive::*;

pub trait Delegatable<'a, T>: Sized {
    type Target;
    fn delegate_mut(&mut self) -> &mut Self::Target;
    fn delegate_ref(&self) -> &Self::Target;
    fn delegate_owned(self) -> Self::Target;
}
