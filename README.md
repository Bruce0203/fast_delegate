# delegare

1. Faster than `auto_delegate` crate 
2. Easier than `deleagate` crate 

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
}
```

# known issue warning
All delegatable traits must be [object safe](https://doc.rust-lang.org/reference/items/traits.html#object-safety).

