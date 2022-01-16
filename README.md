<br/>
<div>
	<a href="https://github.com/kicad-rs/millimeter/actions/workflows/rust.yml">
		<img alt="build status" src="https://github.com/kicad-rs/millimeter/actions/workflows/rust.yml/badge.svg"/>
	</a>
	<a href="https://crates.io/crates/millimeter">
		<img alt="millimeter on crates.io" src="https://img.shields.io/crates/v/millimeter.svg"/>
	</a>
	<a href="https://docs.rs/millimeter">
		<img alt="millimeter on docs.rs" src="https://docs.rs/millimeter/badge.svg"/>
	</a>
	<a href="https://kicad-rs.github.io/millimeter/doc/millimeter/index.html">
		<img alt="rustdoc of main branch" src="https://img.shields.io/badge/docs-main-blue.svg"/>
	</a>
</div>

# millimeter

This crate provides [`mm`][__link0] and [`mm2`][__link1] newtype structs. These can be used both as an indication that a value is expected to have a certain unit, as well as to prove at compile time that your computation yields the unit you expect it to.


## Example


```rust
use millimeter::{mm, mm2, Unit};

#[derive(Clone, Copy, Default)]
pub struct Point {
	x: mm,
	y: mm
}

#[derive(Clone, Copy)]
pub struct Rectangle {
	top_left: Point,
	bottom_right: Point
}

impl Rectangle {
	pub fn one_inch_square(top_left: Point) -> Self {
		Self {
			top_left,
			bottom_right: Point {
				x: top_left.x + 1.0.inch(),
				y: top_left.y + 1.0.inch()
			}
		}
	}

	pub fn area(&self) -> mm2 {
		(self.bottom_right.x - self.top_left.x) * (self.bottom_right.y - self.top_left.y)
	}

	pub fn diagonal_len(&self) -> mm {
		let a = self.bottom_right.x - self.top_left.x;
		let b = self.bottom_right.y - self.top_left.y;
		(a*a + b*b).sqrt()
	}
}
```



## License

Licensed under the [BSD Zero Clause License](./LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate
by you, as defined in the [Apache License, Version 2.0](https://apache.org/licenses/LICENSE-2.0.txt), shall
be licensed as above, without any additional terms or conditions.

 [__cargo_doc2readme_dependencies_hash]: 3EC75BDA2FA622635DE524A794D31270BB2E29844E13EEE2FDC052ECC4C570DC
 [__link0]: https://docs.rs/millimeter/0.0.0/millimeter/?search=mm
 [__link1]: https://docs.rs/millimeter/0.0.0/millimeter/?search=mm2
