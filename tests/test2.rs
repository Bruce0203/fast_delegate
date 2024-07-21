use fast_delegate::{delegate, Delegate};

#[delegate]
pub trait Delegate<T> {
    fn do_something(&mut self);
}

#[derive(Delegate)]
pub struct Delegated {
    #[to(Delegate<usize>)]
    a: MyStruct,
}

pub struct MyStruct;
impl<T> Delegate<T> for MyStruct {
    fn do_something(&mut self) {
        println!("HI");
    }
}
