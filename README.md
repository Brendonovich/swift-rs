# swift-rs

[![Current Crates.io Version](https://img.shields.io/crates/v/swift-rs.svg)](https://crates.io/crates/swift-rs)
[![License](https://img.shields.io/crates/l/swift-rs.svg)](#license)
![Build](https://github.com/Brendonovich/swift-rs/workflows/Build/badge.svg?branch=master)

Call Swift functions from Rust with ease!

## Setup

Add `swift-rs` to your project's `dependencies` and `build-dependencies`:

```toml
[dependencies]
swift-rs = "0.3.0"

[build-dependencies]
swift-rs = { version = "0.3.0", features = ["build"] }
```

Next, some setup work must be done:

1. Ensure your swift code is organized into a Swift Package. This can be done in XCode by selecting File -> New -> Project -> Multiplatform -> Swift Package and importing your existing code.
2. Add `SwiftRs` as a dependency to your Swift package. A quick internet search can show you how to do this.
3. Create a `build.rs` file in your project's source folder, if you don't have one already.
4. Link the swift runtime to your binary

```rust
use swift_rs::build;

fn build() {
    build::link_swift();

    // Other build steps
}
```

5. Link your swift package to your binary. `link_swift_package` takes 2 arguments: The name of your package as specified in its `Package.swift`, and the location of your package's root folder relative to your rust project's root folder.

```rust
use swift_rs::build;

fn build() {
    build::link_swift("10.15" /* macOS Catalina */); // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    build::link_swift_package(PACKAGE_NAME, PACKAGE_PATH);

    // Other build steps
}
```

With those steps completed, you should be ready to start using Swift code from Rust!

### macOS minimum system version

When using `swift-rs` you may to want to use Swift API's which are only available from a certain macOS versions. You can set the minimum version of macOS you intend to support in the `Package.swift` file of your Swift project. `swift-rs` supports `10.10` (OS X Yosemite) and later.

```swift
let package = Package(
    // ...
    platforms: [
        .macOS(.v10_15 /* macOS Catalina */), // This specifies the earliest version of macOS that is supported.
    ],
    // ...
)
```

If you experience the error `dyld[16008]: Library not loaded: @rpath/libswiftCore.dylib` when using `swift-rs` with [Tauri](https://tauri.app) ensure you have set your [Tauri minimum system version](https://tauri.app/v1/guides/distribution/macos/#minimum-system-version) to `10.15` or higher in your `tauri.config.json`. 

## Calling basic functions

To allow calling a Swift function from Rust, it must follow some rules:

1. It must be global
2. It must be annotated with `@_cdecl`, so that it is callable from C
3. It must only use types that can be represented in Objective-C, so only classes that derive `NSObject`, as well as primitives such as Int and Bool. This excludes strings, arrays, generics (though all of these can be sent with workarounds) and structs (which are strictly forbidden).

For this example we will use a function that simply squares a number:

```swift
public func squareNumber(number: Int) -> Int {
    return number * number
}
```

So far, this function meets requirements 1 and 3: It is global and public, and only uses the Int type, which is Objective-C compatible. However, it is not annotated with `@_cdecl`. To fix this, we must call `@_cdecl` before the function's declaration and specify the name that the function is exposed to Rust with as its only argument. To keep with Rust's naming conventions, we will export this function in snake case as `return_number`.

```swift
@_cdecl("square_number")
public func squareNumber(number: Int) -> Int {
    return number * number
}
```

Now that `returnNumber` is properly exposed to Rust, we can start interfacing with it. This is done by pretending that the function is being exposed by a C library, when in reality it is a Swift library exporting a function that looks like it is from C (thanks to `@_cdecl`):

```rust
extern "C" {
    fn square_number(number: usize) -> usize;
}
```

Lastly, you can call the function from regular Rust functions. Note that <b>all</b> calls to a Swift function are unsafe, and require wrapping in an `unsafe {}` block or `unsafe fn`.

```rust
fn main() {
    let input: usize = 4;
    let output = unsafe { square_number(input) };

    println!("Input: {}, Squared: {}", input, output);
    // Prints "Input: 4, Squared: 16"
}
```

For reference, here is a table for most primitive Rust <-> Swift type conversions

|  Rust   |  Swift   |
| :-----: | :------: |
| `usize` |  `UInt`  |
| `isize` |  `Int`   |
|  `iN`   |  `IntN`  |
|  `uN`   | `UIntN`  |
|  `fN`   | `FloatN` |
| `bool`  |  `Bool`  |

## Returning objects from Swift

Let's say that we want our `squareNumber` function to return not only the result, but also the original input. A standard way to do this in Swift would be with a struct:

```swift
struct SquareNumberResult {
    var input: Int
    var output: Int
}
```

We are not allowed to do this, though, since structs cannot be represented in Objective-C. Instead, we must use a class that extends `NSObject`:

```swift
class SquareNumberResult: NSObject {
    var input: Int
    var output: Int

    init(_ input: Int, _ output: Int) {
        self.input = input;
        self.output = output
    }
}
```

<sub><sup>Yes, this class could contain the squaring logic too, but that is irrelevant for this example

An instance of this class can then be returned from `squareNumber` (remember to wrap it up under `toRust()`):

```swift
@_cdecl("square_number")
public func squareNumber(input: Int) -> SquareNumberResult {
    let output = input * input
    return toRust(SquareNumberResult(input, output))
}
```

As you can see, returning an `NSObject` from Swift isn't too difficult. The same can't be said for the Rust implementation, though. `squareNumber` doesn't actually return a struct containing `input` and `output`, but instead a pointer to a `SquareNumberResult` stored somewhere in memory. Additionally, this value contains more data than just `input` and `output`: Since it is an `NSObject`, it contains extra data that must be accounted for when using it in Rust.

This may sound daunting, but it's not actually a problem thanks to `SRObject<T>`. This type manages the pointer internally, and takes a generic argument for a struct that we can access the data through. Let's see how we'd implement `SquareNumbeResult` in Rust:

```rust
// Any struct that is used in a C function must be annotated
// with this, and since our Swift function is exposed as a
// C function with @_cdecl, this is necessary here
#[repr(C)]
// Struct matches the class declaration in Swift
struct SquareNumberResult {
    input: usize,
    output: usize
}

extern "C" {
    // SRObject abstracts away the underlying pointer and will automatically deref to
    // &SquareNumberResult through the Deref trait
    fn square_number(input: usize) -> SRObject<SquareNumberResult>;
}
```

Then, using the new return value is just like using `SquareNumberResult` directly:

```rust
fn main() {
    let input = 4;
    let result = unsafe { square_number(input) };

    let result_input = result.input; // 4
    let result_output = result.output; // 16
}
```

Creating objects in Rust and then passing them to Swift is not supported.

## Optionals

`swift-rs` also supports Swift's `nil` type, but only for functions that return optional `NSObject`s. Functions returning optional primitives cannot be represented in Objective C, and thus are not supported.

Let's say we have a function returning an optional `SRString`:

```swift
@_cdecl("optional_string")
func optionalString(returnNil: Bool) -> SRString? {
    if (returnNil) return nil
    else return SRString("lorem ipsum")
}
```

Thanks to Rust's [null pointer optimisation](https://doc.rust-lang.org/std/option/index.html#representation), the optional nature of `SRString?` can be represented by wrapping `SRString` in Rust's `Option<T>` type!

```rust
extern "C" {
    fn optional_string(return_nil: bool) -> Option<SRString>
}
```

Null pointers are actually the reason why a function that returns an optional primitives cannot be represented in C. If this were to be supported, how could a `nil` be differentiated from a number? It can't!

## Complex types

So far we have only looked at using primitive types and structs/classes, but this leaves out some of the most important data structures: arrays (`SRArray<T>`) and strings (`SRString`). These types must be treated with caution, however, and are not as flexible as their native Swift & Rust counterparts.

### Strings

Strings can be passed between Rust and Swift through `SRString`, which can be created from native strings in either language.

**As an argument**

```swift
import SwiftRs

@_cdecl("swift_print")
public func swiftPrint(value: UnsafePointer<SRString>) {
    // value.pointee gives us the actual SRString instance,
    // and .to_string() converts the SRString to a Swift String
    print(value.pointee.to_string())
}
```

```rust
use swift_rs::types::SRString;

extern "C" {
    fn swift_print(value: &SRString);
}

fn main() {
    // SRString can be created by simply calling into() on any string reference.
    // This will allocate memory in Swift and copy the string
    let value: SRString = "lorem ipsum".into();

    unsafe { swift_print(&value) }; // Will print "lorem ipsum" to the console
}
```

**As a return value**

```swift
import SwiftRs

@_cdecl("get_string")
public func getString() -> SRString {
    let value = "lorem ipsum"

    // SRString can be created from a regular String
    // Again, remember to wrap up the return value under toRust()
    return toRust(SRString(value))
}
```

```rust
use swift_rs::types::SRString;

extern "C" {
    fn get_string() -> SRString;
}

fn main() {
    let value_srstring: SRString = unsafe { get_string() };

    // SRString can be converted to an &str using as_str()...
    let value_str: &str = value_srstring.as_str();
    // or though the Deref trait
    let value_str: &str = &*value_srstring;

    // SRString also implements Display
    println!("{}", value_ststring); // Will print "lorem ipsum" to the console
}
```

### Arrays

**Primitive Arrays**

Representing arrays properly is tricky, since we cannot use generics as Swift arguments or return values according to rule 3. Instead, `swift-rs` provides a generic `SRArray<T>` that can be embedded inside another class that extends `NSObject` that is not generic, but is restricted to a single element type.

```swift
import SwiftRs

// Argument/Return values can contain generic types, but cannot be generic themselves.
// This includes extending generic types.
class IntArray: NSObject {
    var data: SRArray<Int>

    init(_ data: [Int]) {
        self.data = SRArray(data)
    }
}

@_cdecl("get_numbers")
public func getNumbers() -> IntArray {
    let numbers = [1, 2, 3, 4]

    return toRust(IntArray(numbers))
}
```

```rust
use swift_rs::types::{SRArray, SRObject};

#[repr(C)]
struct IntArray {
    data: SRArray<usize>
}

extern "C" {
    // Since IntArray extends NSObject in its Swift implementation,
    // it must be wrapped in SRObject on the Rust side
    fn get_numbers() -> SRObject<IntArray>;
}

fn main() {
    let numbers = unsafe { get_numbers() };

    // SRArray can be accessed as a slice via as_slice...
    let numbers_slice: &[usize] = numbers.data.as_slice();

    // Or though double deref: Once to get past SRObject, another to get past SRArray
    let numbers_slice: &[usize] = &**numbers.data;

    println!("{:?}", numbers_slice); // Will print "[1, 2, 3, 4]" to the console
}
```

To simplify thing on the rust side, however, we can actually do away with the `IntArray` struct. Since `IntArray` only has one field, its memory layout is identical to that of `SRArray<usize>`, so our Rust implementation can be simplified at the cost of equivalence with our Swift code:

```rust
extern "C" {
    // We still need to wrap the array in SRObject since
    // the wrapper class in Swift is an NSObject
    fn get_numbers() -> SRObject<SRArray<usize>>;
}
```

**NSObject Arrays**

What if we want to return an `NSObject` array? There are two options on the Swift side:

1. Continue using `SRArray` and a custom wrapper type, or
2. Use `SRObjectArray`, a wrapper type provided by `swift-rs` that accepts any `NSObject` as its elements. This can be easier than continuing to create wrapper types, but sacrifices some type safety.

There is also `SRObjectArray<T>` for Rust, which is compatible with any single-element Swift wrapper type (and of course `SRObjectArray` in Swift), and automatically wraps its elements in `SRObject<T>`, so there's very little reason to not use it unless you _really_ like custom wrapper types.

Using `SRObjectArray` in both Swift and Rust with a basic custom class/struct can be done like this:

```swift
import SwiftRs

class IntTuple: NSObject {
    var item1: Int
    var item2: Int

    init(_ item1: Int, _ item2: Int) {
       self.item1 = item1
       self.item2 = item2
    }
}

@_cdecl("get_tuples")
public func getTuples() -> SRObjectArray {
    let tuple1 = IntTuple(0,1),
        tuple2 = IntTuple(2,3),
        tuple3 = IntTuple(4,5)

    let tupleArray: [IntTuple] = [
        tuple1,
        tuple2,
        tuple3
    ]

    // Type safety is only lost when the Swift array is converted to an SRObjectArray
    return toRust(SRObjectArray(tupleArray))
}
```

```rust
use swift_rs::types::SRObjectArray;

#[repr(C)]
struct IntTuple {
    item1: usize,
    item2: usize
}

extern "C" {
    // No need to wrap IntTuple in SRObject<T> since
    // SRObjectArray<T> does it automatically
    fn get_tuples() -> SRObjectArray<IntTuple>;
}

fn main() {
    let tuples = unsafe { get_tuples() };

    for tuple in tuples.as_slice() {
        // Will print each tuple's contents to the console
        println!("Item 1: {}, Item 2: {}", tuple.item1, tuple.item2);
    }
}
```

Complex types can contain whatever combination of primitives and `SRObject<T>` you like, just remember to follow the 3 rules!

## Bonuses

### SRData

A wrapper type for `SRArray<T>` designed for storing `u8`s, essentially just a byte buffer.

### Tighter Memory Control with autoreleasepool
If you've come to Swift from an Objective-C background, you likely know the utility of `@autoreleasepool` blocks. `swift_rs` has your back on this too, just wrap
your block of code with a `autoreleasepool!`, and that block of code now executes with its own autorelease pool, e.g.
```rust
use swift_rs::autoreleasepool;

for _ in 0..10000 {
    autoreleasepool!({
        // do some memory intensive thing here
    });
}
```

## Limitations

Currently, the only types that can be used as function arguments are number types, boolean and `SRString`. This is because those types are easy to allocate memory for, either on the stack or on the heap via calling out to swift, whereas other types are not. This may be implemented in the future, though.

Mutating values across Swift and Rust is not currently an aim for this library, it is purely for providing arguments and returning values. Besides, this would go against Rust's programming model, potentially allowing for multiple shared references to a value instead of interior mutability via something like a Mutex.

## Why?

I was helping my friend [Jamie Pine](https://twitter.com/jamiepine) with a desktop app made with [Tauri](https://twitter.com/TauriApps), an Electron alternative that uses Rust as its backend. One of the features Jamie wanted was to get the preview icon for files on his filesystem, which can be done with the [icon(forFile:)](https://developer.apple.com/documentation/appkit/nsworkspace/1528158-icon) function on the app's `NSWorkspace`. This requires accessing the static `shared` property of `NSWorkspace`, something that after some research wasn't possible using the [Rust Objective-C bindings](https://docs.rs/objc/0.2.7/objc/) (since from what I can tell it only supports sending and receiving messages, not accessing static properties), and I couldn't figure out if [swift-bindgen](https://github.com/nvzqz/swift-bindgen) could do the job. So I created this library and the rest is history!

The examples folder is similar to the same Swift code that Jamie uses in his project. While there's probably other, less unsafe ways to interop with Swift, its been both my and Jamie's experience that leveraging Swift for it's native API access and Rust for building applications is quite nice compared to wrangling Swift with calls from Rust similar to how the `objc` crate has you do. This library probably has a littany of problems around memory management and leaks since I'm not that well versed in the Swift runtime, but it gets the job done!

## Todo

- More ease of use and utility functions

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
