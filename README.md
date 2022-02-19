# Maths

My bespoke maths library that I find myself continually reusing. Designed to be lightweight and
easy to use while maintaining compatibility with glsl **std140**. May contain errors...

## Example
```rust
use maths::*;

let vec2 = Vector2 { x: 5.2, y: -9.0 } + Vector2::UP * 5.0;

let rotation = Quaternion::from_euler(Vector3 { x: 60.0, y: 0.0, z: 0.0 });

let mut velocity = Vector3::ZERO;
loop {
    const GRAVITY: Vector3 = Vector3::DOWN * 9.8;
    velocity += GRAVITY * 0.1;
}
```