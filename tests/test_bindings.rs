//! Test for swift-rs bindings
//!
//! Needs to be run with the env var `TEST_SWIFT_RS=true`, to allow for
//! the test swift code to be linked.

use std::{env, process::Command};
use swift_rs::SRString;

#[test]
fn test_string() {
    let name: SRString = "Bond".into();
    let greeting = unsafe { get_greeting(&name) };
    assert_eq!(greeting.as_str(), "Hello Bond");
}

#[test]
fn test_memory_leaks() {
    let leaks_env_var = "TEST_RUNNING_UNDER_LEAKS";
    if env::var(leaks_env_var).unwrap_or_else(|_| "false".into()) == "true" {
        // create memory pressure
        let name: SRString = "Bond".into();
        for _ in 0..10000 {
            let greeting = unsafe { get_greeting(&name) };
            assert_eq!(greeting.as_str(), "Hello Bond");
        }
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
}

extern "C" {
    fn get_greeting(name: &SRString) -> SRString;
}

const DEBUG_PLIST_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict><key>com.apple.security.get-task-allow</key><true/></dict>
</plist>
"#;
