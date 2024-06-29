#![feature(associated_type_defaults)]

use std::marker::PhantomData;

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
    fn run3(&mut self, value: C);
}

#[derive(Delegate)]
pub struct Delegated<T> {
    #[to(Delegate)]
    entity: DelegateImpl,
    #[to(Delegate2)]
    entity2: Delegate2Impl,
    #[to(Delegate3<T>)]
    entity3: Delegate3Impl,
    _marker: PhantomData<T>,
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

pub struct Delegate3Impl;
impl<C> Delegate3<C> for Delegate3Impl
where
    C: Default,
{
    fn run3(&mut self, value: C) {
        println!("hi");
    }
}

#[test]
fn delegate_test() {
    let player = Delegated {
        entity: DelegateImpl {},
        entity2: Delegate2Impl {},
        entity3: Delegate3Impl {},
        _marker: PhantomData::<usize>,
    };
    player.run();
    player.run2(123);
}
