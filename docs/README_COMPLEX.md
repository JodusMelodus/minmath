# Complex

## Features

- **Complex Number type** with float fields
- **Rectangular and Polar** support for every use case
- **Commonly used functions** like `modulus` and `conjugate`

---

## Traits

The following traits are derived for the `Complex` structure:

```rust
Clone
Copy
PartialEq
Eq
```

---

## Construction

### Creating a Complex Number

```rust
pub fn new(re: f32, im: f32) -> Self
pub fn from_rect(x: f32, y: f32) -> Self
pub fn from_polar(r: f32, theta: f32) -> Self
```

Creates a new Complex number from rectangular or polar coordinates.

**Example:**
```rust
let z1 = Complex::new(4.0, -3.0);
let z2 = Complex::from_rect(4.0, -3.0);
let z3 = Complex::from_polar(4.0, -3.0);
```

## Functions

### Modulus

```rust
pub fn r#mod(&self) -> f32
```

Returns the modulus of a complex number.

**Example:**
```rust
let z = Complex::new(4.0, 3.0);
let r = z.r#mod(); // 5.0
```

### Conjugate

```rust
pub fn conj(&self) -> Self
```

Returns the conjugate of a complex number.

**Example:**
```rust
let z1 = Complex::new(4.0, 3.0);
let z2 = z1.conj(); // {4.0, -3.0}
```

---

## See Also

- [Project repository](https://github.com/Jodus-Melodus/minmath)

---
[back](https://github.com/Jodus-Melodus/minmath/blob/master/README.md)