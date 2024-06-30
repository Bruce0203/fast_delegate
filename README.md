# delegare

1. Fastest than other `auto_delegate` crate 
2. Easiest than other `deleagate` crate

# example of this crate

```rust
#[delegate]
pub trait Read {
    fn read(&self) -> &usize;
}

#[derive(Delegate)]
struct Wrapper<R>
where
    R: Read,
{
    #[to(Read)]
    inner: R,
}

struct Io {
    value: usize,
}

impl Read for Io {
    fn read(&self) -> &usize {
        &self.value
    }
}

fn main() {
    let wrapper = Wrapper {
        inner: Io { value: 1 },
    };
    wrapper.read();
}```
