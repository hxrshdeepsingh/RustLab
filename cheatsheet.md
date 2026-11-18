# **Rust Cheat Sheet 🦀**

A practical Rust syntax reference covering beginner to systems programming. Examples use modern Rust 2024 edition syntax.

## **1. Basic program**


```
fn main() {
    println!("Hello, World!");
    print!("Hello again");
}
```

- `fn` = function
- `main()` = entry point
- `println!` = print with newline
- `!` = macro invocation

## **2. Variables**

```
let x = 10;          // Immutable
let mut y = 20;      // Mutable

const MAX: u32 = 100;

let z: i32 = 30;     // Explicit type

let x = "hello";     // Shadowing
```

## **3. Data types**

| Type                              | Description                    |
| --------------------------------- | ------------------------------ |
| `i8`, `i16`, `i32`, `i64`, `i128` | Signed integers                |
| `u8`, `u16`, `u32`, `u64`, `u128` | Unsigned integers              |
| `f32`, `f64`                      | Floating point                 |
| `bool`                            | Boolean                        |
| `char`                            | Unicode character              |
| `str`                             | String slice                   |
| `String`                          | Owned string                   |
| `usize`                           | Pointer-sized unsigned integer |
| `isize`                           | Pointer-sized signed integer   |

```
let age: u32 = 25;
let price: f64 = 99.50;
let active: bool = true;
let letter: char = 'A';
let name: String = String::from("Ishu");
```

## **4. Operators**

```
// Arithmetic
let a = 10 + 5;
let b = 10 - 5;
let c = 10 * 5;
let d = 10 / 5;
let e = 10 % 3;

// Comparison
let x = 10 == 10;
let y = 10 != 5;
let z = 10 > 5;

// Logical
let result = true && false;
let result = true || false;
let result = !true;

// Bitwise
let a = 5 & 3;
let b = 5 | 3;
let c = 5 ^ 3;
let d = 5 << 1;
let e = 5 >> 1;
```

## **5. Conditions**

```
let age = 25;

if age >= 18 {
    println!("Adult");
} else if age >= 13 {
    println!("Teenager");
} else {
    println!("Child");
}
```

Rust `if` is also an expression:

```
let result = if age >= 18 {
    "Adult"
} else {
    "Minor"
};
```

## **6. Loops**

```
// Infinite loop
loop {
    break;
}

// While
let mut i = 0;

while i < 5 {
    println!("{i}");
    i += 1;
}

// For range (0 to 4)
for i in 0..5 {
    println!("{i}");
}

// Inclusive range (0 to 5)
for i in 0..=5 {
    println!("{i}");
}

// Continue
for i in 0..10 {
    if i == 5 {
        continue;
    }
    println!("{i}");
}
```

## **7. Functions**

```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Implicit return
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn greet(name: &str) {
    println!("Hello {name}");
}
```

## **8. Structs**

```
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Ishu"),
        age: 25,
    };

    println!("{}", user.name);
}
```

Methods:

```
impl User {
    fn greet(&self) {
        println!("Hello {}", self.name);
    }

    fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}
```

## **9. Enums**

```
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;
```

Enums with data:

```
enum Message {
    Text(String),
    Number(i32),
    Quit,
}
```

## **10. Match**

```
let number = 2;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Other"),
}
```

## **11. Strings**

```
let name = String::from("Ishu");

let borrowed: &str = &name;

let mut text = String::new();

text.push_str("Hello");
text.push('!');

println!("{}", text);
println!("Length: {}", text.len());
```

String formatting:

```
let name = "Ishu";
let age = 25;

let message = format!("{name} is {age}");
```

## **12. Arrays and vectors**

```
// Fixed-size array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// Vector (dynamic)
let mut values: Vec<i32> = Vec::new();

values.push(10);
values.push(20);

values.pop();

for value in &values {
    println!("{value}");
}
```

## **13. Tuples**

```
let user: (&str, u32) = ("Ishu", 25);

println!("{}", user.0);
println!("{}", user.1);

let (name, age) = user;
```

## **14. Pointers and references**

This is one of the most important sections for systems programming.

```
let x = 100;

// Reference
let reference = &x;

// Mutable reference
let mut value = 10;
let reference = &mut value;

*reference = 20;

// Raw pointers
let ptr: *const i32 = &x;
let mut y = 10;
let ptr_mut: *mut i32 = &mut y;
```

Raw pointer dereferencing requires `unsafe`:

```
unsafe {
    println!("{}", *ptr);
}
```

## **15. Ownership and borrowing**

```
let a = String::from("Hello");

let b = a; // Ownership moves to b

// a cannot be used here

let c = String::from("World");

let d = &c; // Borrow
println!("{d}");

let mut text = String::from("Hello");

let reference = &mut text;
reference.push_str(" Rust");
```

**Three important rules:**

1. Each value has one owner.
2. When the owner goes out of scope, its value is dropped.
3. You can have multiple immutable references or one mutable reference at a time (with additional lifetime and scope rules).

## **16. Heap allocation**

```
// Stack value
let x = 10;

// Heap allocation
let y = Box::new(100);

// Reference-counted ownership
use std::rc::Rc;

let a = Rc::new(10);
let b = Rc::clone(&a);

// Thread-safe reference counting
use std::sync::Arc;

let shared = Arc::new(100);
```

## **17. Error handling**

```
// Result
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".into());
    }

    Ok(a / b)
}

fn main() {
    match divide(10, 2) {
        Ok(value) => println!("{value}"),
        Err(error) => println!("{error}"),
    }
}
```

The `?` operator:

```
fn operation() -> Result<(), std::io::Error> {
    let data = std::fs::read("file.txt")?;
    println!("{:?}", data);

    Ok(())
}
```

## **18. HashMap**

```
use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert("name", "Ishu");
    users.insert("role", "Developer");

    println!("{:?}", users.get("name"));
}
```

## **19. Traits**

Traits define shared behavior.

```
trait Animal {
    fn sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn sound(&self) {
        println!("Bark");
    }
}
```

## **20. Generics**

```
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    println!("{}", add(10, 20));
}
```

## **21. Closures**

```
let add = |a: i32, b: i32| a + b;

println!("{}", add(10, 20));
```

With iterators:

```
let numbers = vec![1, 2, 3, 4, 5];

let result: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .filter(|x| *x > 4)
    .collect();
```

## **22. Modules**

```
mod network {
    pub fn start() {
        println!("Server started");
    }
}

fn main() {
    network::start();
}
```

Import:

```
use std::net::UdpSocket;
use std::io;
```

## **23. Threads and concurrency**

```
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Worker thread");
    });

    handle.join().unwrap();
}
```

Shared atomic variable:

```
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn main() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
}
```

## **24. UDP networking**

A simple UDP server:

```
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?;

    let mut buffer = [0u8; 1024];

    loop {
        let (size, client) = socket.recv_from(&mut buffer)?;

        println!(
            "Received: {}",
            String::from_utf8_lossy(&buffer[..size])
        );

        socket.send_to(b"Hello from Rust!", client)?;
    }
}
```

## **25. Unsafe Rust**

For low-level programming:

```
fn main() {
    let mut x = 10;

    let ptr = &mut x as *mut i32;

    unsafe {
        *ptr = 100;
    }

    println!("{x}");
}
```

`unsafe` allows operations the compiler cannot verify for safety. It does not disable all Rust safety checks.

## **26. Macros**

```
println!("Hello");
format!("Age: {}", 25);
vec![1, 2, 3];
panic!("Something went wrong");
assert!(10 > 5);
dbg!(100);
```

## **27. Cargo commands**

| Command                 | Purpose                |
| ----------------------- | ---------------------- |
| `cargo new app`         | Create project         |
| `cargo init`            | Initialize project     |
| `cargo run`             | Compile and run        |
| `cargo build`           | Debug build            |
| `cargo build --release` | Optimized build        |
| `cargo check`           | Fast type checking     |
| `cargo test`            | Run tests              |
| `cargo clean`           | Remove build files     |
| `cargo add tokio`       | Add dependency         |
| `cargo update`          | Update dependencies    |
| `cargo tree`            | Dependency tree        |
| `cargo doc --open`      | Generate documentation |

## **28. Optimized release binaries**

For a small, optimized executable, edit `Cargo.toml`:

```
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

Build:

```
cargo build --release
```

For maximum performance rather than minimum size:

```
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

These settings involve trade-offs: size optimization can reduce speed, and LTO increases build time.

## **Quick reference: Rust symbols**

| Syntax         | Meaning                     |
| -------------- | --------------------------- |
| `let`          | Immutable binding           |
| `let mut`      | Mutable binding             |
| `const`        | Compile-time constant       |
| `fn`           | Function                    |
| `pub`          | Public visibility           |
| `impl`         | Implement methods/traits    |
| `struct`       | Structure                   |
| `enum`         | Enum                        |
| `trait`        | Shared behavior             |
| `&T`           | Immutable reference         |
| `&mut T`       | Mutable reference           |
| `*const T`     | Raw immutable pointer       |
| `*mut T`       | Raw mutable pointer         |
| `Box<T>`       | Heap ownership              |
| `Arc<T>`       | Atomic reference counting   |
| `Option<T>`    | Optional value              |
| `Result<T, E>` | Success or error            |
| `match`        | Pattern matching            |
| `loop`         | Infinite loop               |
| `?`            | Propagate errors            |
| `move`         | Transfer captures/ownership |
| `unsafe`       | Explicit unsafe operations  |
| `where`        | Generic constraints         |
| `::`           | Path separator              |
| `->`           | Return type                 |
| `!`            | Macro invocation            |
