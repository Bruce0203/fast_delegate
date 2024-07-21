fn main() {
    let d = Delegated {
        value: DelegateImpl,
    };
    d.do_it();
}
struct DelegateImpl;
trait Delegate {
    type Error;
    fn do_it(&self);
}
impl<'__delegate_lifetime: 'static, __DelegateImpl> Delegate for __DelegateImpl
where
    __DelegateImpl: fast_delegate::Delegatable<
        '__delegate_lifetime,
        &'__delegate_lifetime dyn Delegate<Error = ()>,
    >,
    __DelegateImpl::Target: Delegate,
{
    type Error = ();
    #[inline(always)]
    fn do_it(&self) {
        self.delegate_ref().do_it()
    }
}
impl Delegate for DelegateImpl {
    fn do_it(&self) {
        {
            println!("hi");
        };
    }

    type Error = usize;
}
struct Delegated {
    //#[to(Delegate)]
    value: DelegateImpl,
}
impl<'__delegate_lifetime: 'static>
    fast_delegate::Delegatable<'__delegate_lifetime, &'__delegate_lifetime dyn Delegate<Error = ()>>
    for Delegated
{
    type Target = DelegateImpl;
    fn delegate_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
    fn delegate_ref(&self) -> &Self::Target {
        &self.value
    }
    fn delegate_owned(self) -> Self::Target {
        self.value
    }
}
