//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
// use std::env;

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?

    let your_command = format!(
        "cargo:TEST_FOO={}",
        timestamp
    );
    // let your_command = format!(
    //     "Your command here with {}, please checkout exercises/tests/build.rs",
    //     timestamp
    // );

    // let rustc_version = env::var("CARGO_PKG_VERSION").unwrap();
    // println!("Rustc version: {}", your_command);
    println!("cargo:{}", your_command);

    let your_command = format!(
        "cargo:rustc-cfg=feature[={}]",
        "pass"
    );

    println!("cargo:{}", your_command);
    // let test_command = format!(
    //     "cargo:rustc-link-arg-tests=TEST_FOO={}",
    //     timestamp
    // );
    // println!("cargo:{}", test_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    // let your_command = "Your command here, please checkout exercises/tests/build.rs";
    // println!("cargo:{}", your_command);
}
