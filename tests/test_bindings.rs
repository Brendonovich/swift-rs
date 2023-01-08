//! Test for swift-rs bindings
//!
//! Needs to be run with the env var `TEST_SWIFT_RS=true`, to allow for
//! the test swift code to be linked.

use swift_rs::SRString;

#[test]
fn test_string() {
    let greeting = unsafe { get_greeting("Bond".into()) };
    assert_eq!(greeting.as_str(), "Hello Bond");
}

extern "C" {
    fn get_greeting(name: SRString) -> SRString;
}
