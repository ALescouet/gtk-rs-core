// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

extern crate pkg_config;

use pkg_config::{Config, Error};
use std::env;
use std::io::prelude::*;
use std::io;
use std::process;

fn main() {
    if let Err(s) = find() {
        let _ = writeln!(io::stderr(), "{}", s);
        process::exit(1);
    }
}

fn find() -> Result<(), Error> {
    let package_name = "gio-2.0";
    let shared_libs = ["gio-2.0"];
    let version = if cfg!(feature = "v2_58") {
        "2.58"
    } else if cfg!(feature = "v2_56") {
        "2.56"
    } else if cfg!(feature = "v2_54") {
        "2.54"
    } else if cfg!(feature = "v2_52") {
        "2.52"
    } else if cfg!(feature = "v2_50") {
        "2.50"
    } else if cfg!(feature = "v2_48") {
        "2.48"
    } else if cfg!(feature = "v2_46") {
        "2.46"
    } else if cfg!(feature = "v2_44") {
        "2.44"
    } else {
        "2.42"
    };

    if let Ok(inc_dir) = env::var("GTK_INCLUDE_DIR") {
        println!("cargo:include={}", inc_dir);
    }
    if let Ok(lib_dir) = env::var("GTK_LIB_DIR") {
        for lib_ in shared_libs.iter() {
            println!("cargo:rustc-link-lib=dylib={}", lib_);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir);
        return Ok(())
    }

    let target = env::var("TARGET").expect("TARGET environment variable doesn't exist");
    let hardcode_shared_libs = target.contains("windows");

    let mut config = Config::new();
    config.atleast_version(version);
    config.print_system_libs(false);
    if hardcode_shared_libs {
        config.cargo_metadata(false);
    }
    match config.probe(package_name) {
        Ok(library) => {
            if let Ok(paths) = std::env::join_paths(library.include_paths) {
                println!("cargo:include={}", paths.to_string_lossy());
            }
            if hardcode_shared_libs {
                for lib_ in shared_libs.iter() {
                    println!("cargo:rustc-link-lib=dylib={}", lib_);
                }
                for path in library.link_paths.iter() {
                    println!("cargo:rustc-link-search=native={}",
                             path.to_str().expect("library path doesn't exist"));
                }
            }
            Ok(())
        }
        Err(Error::EnvNoPkgConfig(_)) | Err(Error::Command { .. }) => {
            for lib_ in shared_libs.iter() {
                println!("cargo:rustc-link-lib=dylib={}", lib_);
            }
            Ok(())
        }
        Err(err) => Err(err),
    }
}

