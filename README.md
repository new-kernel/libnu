# LibNU

Novusk User Library - A library for developing userspace applications for [Novusk](https://github.com/new-kernel/novusk)

```rust
#[macro_use] extern crate libnu;
use libnu::io;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!("Print from LibNU!");
}
```
