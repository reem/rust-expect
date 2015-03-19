# expect

> A flexible library for adding assertions to types.

## Example

```rust
extern crate expect;

use std::default::Default;
use expect::{Expect, Assertion};

#[derive(Default)]
struct Point {
    x: f64,
    y: f64
}

// Clockwise from the top left.
#[derive(Default)]
struct Square(Point, Point, Point, Point);

#[derive(Default)]
struct Contains(Point);

impl Assertion<Square> for Contains {
    fn assert(self, square: &Square) {
        assert!(self.0.x > square.0.x && self.0.x < self.1.x
             && self.0.y > square.2.y && self.1.y < self.1.y);
    }
}

fn main() {
    let square = Square(
        Point { x: 0.0, y: 1.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
    );


    square
        .expect(Contains(Point { x: 0.5, y: 0.18 }))
        .expect(Contains(Point { x: 0.12, y: 0.9 }))

        // You can also use tuples of Assertions.
        .expect((Contains(Point { x: 0.63, y: 0.4 }),
                 Contains(Point { x: 0.7, y: 0.85 })));
}
```

Note that even though this example is a single file/crate, the true power
of using `Assertion`s and `expect` is that `Assertion`s can be defined in
a separate crate.

## Usage

Use the crates.io repository; add this to your `Cargo.toml` along
with the rest of your dependencies:

```toml
[dependencies]
expect = "*"
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and maintainer
of expect.

## License

MIT

