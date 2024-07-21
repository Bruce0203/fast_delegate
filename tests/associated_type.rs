use fast_delegate::{delegate, Delegate};

struct DelegateImpl;

#[delegate]
trait Delegate {
    type E;
    fn do_it(&self);
}

impl Delegate for DelegateImpl {
    fn do_it(&self) {
        println!("hi");
    }

    type E = usize;
}

#[derive(Delegate)]
struct Delegated {
    #[to(Delegate<E = ()>)]
    value: DelegateImpl,
}

fn main() {
    let delegated = Delegated {
        value: DelegateImpl,
    };
    delegated.do_it();
}
