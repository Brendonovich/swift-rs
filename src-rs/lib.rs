//! Call Swift functions from Rust with ease!
//!
//! `swift-rs` provides built-time and run-time utilities for interacting with the Swift runtime and calling Swift
//! functions.
//!
//! ## Setup
//!
//! Add `swift-rs` to your project:
//!
//! ```bash
//! cargo add swift-rs
//! cargo add swift-rs --build --features build
//! ```
//!
//! Next, some work must be done:
//! 1. Ensure your swift code is organized into a Swift Package.
//! This can be done in XCode by selecting File -> New -> Project -> Multiplatform -> Swift Package and importing your existing code.
//! 2. Add `SwiftRs` as a dependency to your Swift package.
//! A quick internet search can show you how to do this.
//! 3. Create a `build.rs` file in your project's root folder, if you don't have one already.
//! 4. Use [`SwiftLinker`] in your `build.rs` file to link both the Swift runtime and your Swift package.
//!
#![cfg_attr(docsrs, feature(doc_cfg))]

mod autorelease;
mod swift;
mod swift_arg;
mod swift_ret;
pub mod types;

pub use autorelease::*;
pub use swift::*;
pub use swift_arg::*;
pub use swift_ret::*;
pub use types::*;

#[cfg(feature = "build")]
#[cfg_attr(docsrs, doc(cfg(feature = "build")))]
mod build;
#[cfg(feature = "build")]
pub use build::*;
