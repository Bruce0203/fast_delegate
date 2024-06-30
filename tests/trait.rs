#![feature(associated_type_defaults)]

use std::{fmt::Debug, marker::PhantomData};

use delegare::{delegate, Delegate};

#[delegate]
pub trait Delegate {
    fn run(&self);
}

#[delegate]
pub trait Delegate2 {
    fn run2(&self, value: usize) -> usize;
    fn run2_mut(&mut self, value: usize) -> usize;
}

#[delegate]
pub trait Delegate3<C>
where
    C: Default,
{
    fn run3(&mut self, value: C) -> &usize;
}

#[derive(Delegate)]
pub struct Delegated<T>
where
    T: Default,
{
    #[to(Delegate, AnotherTrait<T>, SomeTrait)]
    entity: DelegateImpl,
    #[to(Delegate2)]
    entity2: Delegate2Impl,
    #[to(Delegate3<T>)]
    entity3: Delegate3Impl<T>,
    _marker: PhantomData<T>,
}

#[delegate]
pub trait SomeTrait {
    fn qwer(&self);
}

impl SomeTrait for DelegateImpl {
    fn qwer(&self) {
        println!("some trait ");
    }
}

#[delegate]
pub trait AnotherTrait<T> {
    fn asdf(&self);
}

impl<T> AnotherTrait<T> for DelegateImpl {
    fn asdf(&self) {
        println!("another trait");
    }
}

pub struct DelegateImpl;
impl Delegate for DelegateImpl {
    fn run(&self) {
        println!("Delegate");
    }
}

pub struct Delegate2Impl;
impl Delegate2 for Delegate2Impl {
    fn run2(&self, value: usize) -> usize {
        println!("Delegate2");
        1
    }

    fn run2_mut(&mut self, value: usize) -> usize {
        println!("hi");
        1123
    }
}

pub struct Delegate3Impl<T>(PhantomData<T>);
impl<C> Delegate3<C> for Delegate3Impl<C>
where
    C: Default,
{
    fn run3(&mut self, value: C) -> &usize {
        println!("hi");
        &1
    }
}

#[cfg(test)]
mod test {
    use std::marker::PhantomData;

    use crate::{Delegate, Delegate2, Delegate3};
    use crate::{Delegate2Impl, Delegate3Impl, DelegateImpl, Delegated};

    #[test]
    fn delegate_test() {
        let mut player = Delegated {
            entity: DelegateImpl {},
            entity2: Delegate2Impl {},
            _marker: PhantomData::<usize>,
            entity3: Delegate3Impl(PhantomData),
        };
        player.run();
        player.run2(123);
        player.run3(123);
        crate::AnotherTrait::<usize>::asdf(&player);
    }
}
