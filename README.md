# delegare

# example of this crate

```rust
use delegare::{delegate, Delegate};

#[delegate]
pub trait Delegate {
    fn run(&self);
}

#[delegate]
pub trait Delegate2 {
    fn run2(&self);
}

#[derive(Delegate)]
pub struct Delegated {
    #[to(Delegate)]
    entity: DelegateImpl,
    #[to(Delegate2)]
    entity2: Delegate2Impl,
}

pub struct DelegateImpl;
impl Delegate for DelegateImpl {
    fn run(&self) {
        println!("Delegate");
    }
}

pub struct Delegate2Impl;
impl Delegate2 for Delegate2Impl {
    fn run2(&self) {
        println!("Delegate2");
    }
}

#[test]
fn delegate_test() {
    let player = Delegated {
        entity: DelegateImpl {},
        entity2: Delegate2Impl {},
    };
    player.run();
    player.run2();
}
```
