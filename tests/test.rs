mod core_libs {
    pub trait Delegatable<'__delegate_lifetime, Token> {
        type Target;
        fn delegate_ref(&self) -> &Self::Target;
    }
}

mod user_defined_impls_0 {
    use std::marker::PhantomData;

    pub trait MyTrait<T> {
        fn run(&self);
    }

    pub trait MyTrait2 {
        fn run2(&self);
    }

    pub trait MyTrait3 {
        fn run3(&self);
    }

    pub struct Delegate;

    impl<T> MyTrait<T> for Delegate {
        fn run(&self) {
            println!("my trait 1");
        }
    }
    impl MyTrait2 for Delegate {
        fn run2(&self) {
            println!("my trait 2");
        }
    }
    impl MyTrait3 for Delegate {
        fn run3(&self) {
            println!("my trait 3");
        }
    }

    pub struct Delegated<T>(pub Delegate, pub PhantomData<T>);
}

mod macro_generated_impls {

    use crate::{
        core_libs::Delegatable,
        user_defined_impls_0::{Delegate, Delegated, MyTrait, MyTrait2, MyTrait3},
    };

    impl<'a, DelegateImpl, T> MyTrait<T> for DelegateImpl
    where
        DelegateImpl: Delegatable<'a, &'a dyn MyTrait<T>>,
        DelegateImpl::Target: MyTrait<T>,
        T: 'a,
    {
        fn run(&self) {
            self.delegate_ref().run()
        }
    }

    impl<'a, DelegateImpl> MyTrait2 for DelegateImpl
    where
        DelegateImpl: Delegatable<'a, &'a dyn MyTrait2>,
        DelegateImpl::Target: MyTrait2,
    {
        fn run2(&self) {
            self.delegate_ref().run2()
        }
    }

    impl<'a, DelegateImpl> MyTrait3 for DelegateImpl
    where
        DelegateImpl: Delegatable<'a, &'a dyn MyTrait3>,
        DelegateImpl::Target: MyTrait3,
    {
        fn run3(&self) {
            self.delegate_ref().run3()
        }
    }

    impl<'a, T> Delegatable<'a, &'a dyn MyTrait<T>> for Delegated<T> {
        type Target = Delegate;
        fn delegate_ref(&self) -> &Delegate {
            &self.0
        }
    }

    impl<'a, T> Delegatable<'a, &'a dyn MyTrait2> for Delegated<T> {
        type Target = Delegate;
        fn delegate_ref(&self) -> &Delegate {
            &self.0
        }
    }

    impl<'a, T> Delegatable<'a, &'a dyn MyTrait3> for Delegated<T> {
        type Target = Delegate;
        fn delegate_ref(&self) -> &Delegate {
            &self.0
        }
    }
}

#[cfg(test)]
mod test {
    use crate::user_defined_impls_0::{Delegate, Delegated, MyTrait, MyTrait2, MyTrait3};

    #[test]
    fn test() {
        let delegated = Delegated(Delegate, std::marker::PhantomData::<i32>);
        delegated.run();
        delegated.run2();
        delegated.run3();
    }
}
