#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]

use mathplus::function;


pub fn composite_example_0() {
	let linear_fn: impl Fn(f32) -> f32 = move |x: f32| {
			2.0 * x + 4.0
	};

	let pow_2_fn: impl Fn(f32) -> f32 = move |x: f32| {
			x * x
	};

	let y = function::composite(linear_fn, pow_2_fn);


	for x in 0..=10 {
		println!("x = {:?}; y(x) = {:?}", x as f32, y(x as f32));
	}
}

pub fn main() {
	println!("Composite Example 0");
	println!("===================");
	println!("f(x)    = 2.0 * x + 4.0");
	println!("g(x)    = x * x");
	println!("f(g(x)) = 2.0 (x * x) + 4.0");
	println!("---------------------------");
	composite_example_0();
	println!();
}