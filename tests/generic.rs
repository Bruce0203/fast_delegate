use fast_delegate::{delegate, Delegate};

fn main() {
    //let delegated2 = Delegated2 {
    //    delegated: Delegated {
    //        value: DelegateImpl,
    //    },
    //};
    //delegated2.do_it();
}

struct DelegateImpl;

#[delegate]
trait Delegate {
    fn do_it(&self);
}

impl Delegate for DelegateImpl {
    fn do_it(&self) {
        println!("hi");
    }
}

#[derive(Delegate)]
struct Delegated<T> {
    value: T,
}

//#[derive(Delegate)]
//struct Delegated2 {
//    #[to(Delegate(self.value))]
//    delegated: Delegated<DelegateImpl>,
//}
