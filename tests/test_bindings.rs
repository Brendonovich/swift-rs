//! Test for swift-rs bindings
//!
//! Needs to be run with the env var `TEST_SWIFT_RS=true`, to allow for
//! the test swift code to be linked.

use serial_test::serial;
use std::{env, process::Command};
use swift_rs::{autoreleasepool, SRString, SwiftRef, ToSwift};

macro_rules! test_with_leaks {
    ( $op:expr ) => {{
        let leaks_env_var = "TEST_RUNNING_UNDER_LEAKS";
        if env::var(leaks_env_var).unwrap_or_else(|_| "false".into()) == "true" {
            let _ = $op;
        } else {
            // run the above codepath under leaks monitoring
            let exe = env::current_exe().unwrap();

            // codesign the binary first, so that leaks can be run
            let debug_plist = exe.parent().unwrap().join("debug.plist");
            let plist_path = &debug_plist.to_string_lossy();
            std::fs::write(&debug_plist, DEBUG_PLIST_XML.as_bytes()).unwrap();
            let status = Command::new("codesign")
                .args([
                    "-s",
                    "-",
                    "-v",
                    "-f",
                    "--entitlements",
                    plist_path,
                    &exe.to_string_lossy(),
                ])
                .status()
                .expect("cmd failure");
            assert!(status.success(), "failed to codesign");

            // run leaks command to detect memory leaks
            let status = Command::new("leaks")
                .args(["-atExit", "--", &exe.to_string_lossy(), "--nocapture"])
                .env(leaks_env_var, "true")
                .status()
                .expect("cmd failure");
            assert!(status.success(), "leaks detected in memory pressure test");
        }
    }};
}

#[test]
#[serial]
fn test_string() {
    test_with_leaks!(|| {
        let name: SRString = "Bond".into();
        let greeting = unsafe { get_greeting(name.to_swift()) };
        assert_eq!(greeting.as_str(), "Hello Bond");
    });
}

#[test]
#[serial]
fn test_reflection() {
    test_with_leaks!(|| {
        // create memory pressure
        let name: SRString = "Bond".into();
        for _ in 0..10000 {
            let reflected = unsafe { reflect_string(name.to_swift()) };
            assert_eq!(name.as_str(), reflected.as_str());
        }
    });
}

#[test]
#[serial]
fn test_memory_pressure() {
    test_with_leaks!(|| {
        // create memory pressure
        let name: SRString = "Bond".into();
        for _ in 0..10000 {
            let greeting = unsafe { get_greeting(name.to_swift()) };
            assert_eq!(greeting.as_str(), "Hello Bond");
        }
    });
}

#[test]
#[serial]
fn test_autoreleasepool() {
    test_with_leaks!(|| {
        // create memory pressure
        let name: SRString = "Bond".into();
        for _ in 0..10000 {
            autoreleasepool!({
                let greeting = unsafe { get_greeting(name.to_swift()) };
                assert_eq!(greeting.as_str(), "Hello Bond");
            });
        }
    });
}

extern "C" {
    fn get_greeting(name: SwiftRef<SRString>) -> SRString;
    fn reflect_string(string: SwiftRef<SRString>) -> SRString;
}

const DEBUG_PLIST_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict><key>com.apple.security.get-task-allow</key><true/></dict>
</plist>
"#;
