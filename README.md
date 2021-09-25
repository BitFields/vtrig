# mathplus - extended math libary

## Composite functions taking one value and returning one value

```txt
y(x) = f(g(x))
```

```rust
let f: impl Fn(f32) -> f32 = move |x: f32| 2.0 * x + 4.0;
let g: impl Fn(f32) -> f32 = move |x: f32| x * x;

let y: impl Fn(f32) -> f32 = mathplus::composite::function(f, g);

for x in 0..=10 {
    println!("x = {}; y(x) = {}", x, y(x as f32));
}
```
