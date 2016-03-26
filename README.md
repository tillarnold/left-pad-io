#left-pad-io

This is a REST client for http://left-pad.io.
And yes this is kind of a joke.
And I also wanted to try out rust, cargo and crates.io.

What I found out during this little project is, that rusts design really
forces you to think about every possible error. You have to handle every
possible situation where something could go wrong. I quite like that.

I'm sure that the code is quite bad and I plan to improve this
once I learned more about rust. So yeah the API is probably going
to change but nobody should be using this anyways...



```rust
extern crate left_pad_io;
use left_pad_io::left_pad;

assert_eq!("##########hello", left_pad("hello" , "#" , 15).unwrap())

```
