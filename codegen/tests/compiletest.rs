extern crate compiletest_rs;

use std::path::PathBuf;

pub use self::compiletest_rs::common::Mode;
pub use self::compiletest_rs::{Config, run_tests};

pub fn run(mode: Mode) {
    let mut config = Config::default();
    config.mode = mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    let flags = [
        "-L crate=../target/debug/",
        "-L dependency=../target/debug/deps/",
    ].join(" ");

    config.target_rustcflags = Some(flags);
    run_tests(&config);
}
