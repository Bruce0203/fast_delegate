use fast_delegate::{delegate, Delegate};

struct DelegateImpl;

#[delegate]
trait Delegating<const N: usize> {
    fn do_it(&self);
}

impl<const N: usize> Delegating<N> for DelegateImpl {
    fn do_it(&self) {
        println!("hi2");
    }
}

#[derive(Delegate)]
struct Delegated {
    #[to(Delegating<1>)]
    value: DelegateImpl,
}

#[test]
fn main() {
    let delegated = Delegated {
        value: DelegateImpl,
    };
    delegated.do_it();
}
