use std::{
    env,
    process::{exit, Command},
};

use cmake::Config;

fn main() {
    // First we generate .cc and .h files from ffi.rs
    if !cfg!(windows) {
        println!("cargo:warning=No MacOS support yet.");
        exit(0);
    }

    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();
    let tinyinst = format!("{}/TinyInst", &cwd);
    println!("cargo:warning=Pulling TinyInst from github");

    println!("cargo:warning=Generating Bridge files.");
    // Get tinyinst from git
    Command::new("cmd")
        .arg("/C")
        .arg(format!("{cwd}/build.bat"))
        .status()
        .unwrap();

    // source
    Command::new("cxxbridge")
        .args(["src/tinyinst.rs", "-o"])
        .arg(format!("{tinyinst}/bridge.cc"))
        .status()
        .unwrap();

    // header
    Command::new("cxxbridge")
        .args(["src/tinyinst.rs", "--header", "-o"])
        .arg(format!("{tinyinst}/bridge.h"))
        .status()
        .unwrap();

    // cxx
    Command::new("cxxbridge")
        .args(["--header", "-o"])
        .arg(format!("{tinyinst}/cxx.h"))
        .status()
        .unwrap();

    // shim
    std::fs::copy("./src/shim.cc", "./Tinyinst/shim.cc").unwrap();
    std::fs::copy("./src/shim.h", "./Tinyinst/shim.h").unwrap();

    // runresult
    std::fs::copy("./src/runresult.h", "./Tinyinst/runresult.h").unwrap();

    // instrumentation
    std::fs::copy(
        "./src/instrumentation.cpp",
        "./Tinyinst/instrumentation.cpp",
    )
    .unwrap();
    std::fs::copy("./src/instrumentation.h", "./Tinyinst/instrumentation.h").unwrap();

    // tinyinstinstrumentation
    std::fs::copy(
        "./src/tinyinstinstrumentation.cpp",
        "./Tinyinst/tinyinstinstrumentation.cpp",
    )
    .unwrap();
    std::fs::copy(
        "./src/tinyinstinstrumentation.h",
        "./Tinyinst/tinyinstinstrumentation.h",
    )
    .unwrap();

    // aflcov
    std::fs::copy("./src/aflcov.cpp", "./Tinyinst/aflcov.cpp").unwrap();
    std::fs::copy("./src/aflcov.h", "./Tinyinst/aflcov.h").unwrap();

    let dst = Config::new("TinyInst")
        .generator("Visual Studio 17 2022") // make this configurable from env variable
        .build_target("tinyinst")
        .profile("Release") // without this, it goes into RelWithDbInfo folder??
        .build();

    println!("cargo:warning={}", dst.display());
    println!("cargo:rustc-link-search={}\\build\\Release", dst.display()); // debug build?
    println!(
        "cargo:rustc-link-search={}\\build\\third_party\\obj\\wkit\\lib",
        dst.display()
    ); //xed

    println!("cargo:rustc-link-lib=static=tinyinst");
    println!("cargo:rustc-link-lib=static=xed");
    println!("cargo:rustc-link-lib=dylib=dbghelp");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=src/tinyinst.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Tinyinst/litecov.cpp");
}
