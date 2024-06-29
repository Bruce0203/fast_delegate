#![feature(negative_impls)]
#![feature(auto_traits)]
use std::str::FromStr;

use delegare_derive::{delegate, Delegate};

pub trait Delegatable<T> {
    type Target;
    fn delegate_mut(&mut self) -> &mut Self::Target;
    fn delegate_ref(&self) -> &Self::Target;
    fn delegate_owned(self) -> Self::Target;
}

pub trait InternalAPI<Token> {}

pub struct __InternalDelegateToken;
impl<T> Delegate for T
where
    T: Delegatable<__InternalDelegateToken>,
    T::Target: Delegate,
{
    #[inline(always)]
    fn run(&self) {
        return self.delegate_ref().run();
    }
}

impl<T> InternalAPI<__InternalDelegateToken> for T where T: Delegate {}

pub struct InternalDelegate2Token;
impl<T> Delegate2 for T
where
    T: Delegatable<InternalDelegate2Token>,
    T::Target: Delegate2,
{
    #[inline(always)]
    fn run2(&self) {
        return self.delegate_ref().run2();
    }
}

impl<T> InternalAPI<InternalDelegate2Token> for T where T: Delegate2 {}

#[delegate]
pub trait Delegate {
    fn run(&self);
}

pub struct DelegateImpl {
    message: String,
}

#[delegate]
pub trait Delegate2 {
    fn run2(&self);
}

pub struct Delegate2Impl {}

impl Delegate for DelegateImpl {
    fn run(&self) {
        println!("run1: {}", self.message);
    }
}

impl Delegate2 for Delegate2Impl {
    fn run2(&self) {
        println!("asdf");
    }
}

#[derive(Delegate)]
pub struct Delegated {
    #[to(Delegate)]
    entity: DelegateImpl,
    #[to(Delegate2)]
    entity2: Delegate2Impl,
}

impl Delegatable<__InternalDelegateToken> for Delegated {
    type Target = DelegateImpl;

    fn delegate_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }

    fn delegate_ref(&self) -> &Self::Target {
        &self.entity
    }

    fn delegate_owned(self) -> Self::Target {
        self.entity
    }
}

impl Delegatable<InternalDelegate2Token> for Delegated {
    type Target = Delegate2Impl;

    fn delegate_mut(&mut self) -> &mut Self::Target {
        &mut self.entity2
    }

    fn delegate_ref(&self) -> &Self::Target {
        &self.entity2
    }

    fn delegate_owned(self) -> Self::Target {
        self.entity2
    }
}

fn main() {
    let player = Delegated {
        entity: DelegateImpl {
            message: String::from_str("hi!").unwrap(),
        },
        entity2: Delegate2Impl {},
    };
    player.run();
}
