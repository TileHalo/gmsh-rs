// Build script taken and modernized from https://github.com/mxxo/gmsh-sys

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // 1. environment variable takes first priority
    let env_gmsh = "GMSH_LIB_DIR";
    if let Some(lib_dir) = env::var_os(env_gmsh) {
        // rerun if changed
        println!("cargo:rerun-if-env-changed={env_gmsh}");

        let lib_dir = Path::new(&lib_dir);
        let dylib_name = format!("{}gmsh{}", env::consts::DLL_PREFIX, env::consts::DLL_SUFFIX);
        if lib_dir.join(dylib_name).exists() || lib_dir.join("gmsh.lib").exists() {
            println!("cargo:rustc-link-search={}", lib_dir.display());
            // blocked on https://github.com/rust-lang/cargo/issues/4895
            // println!("cargo:rustc-dynamic-link-search={}", lib_dir.display())
        } else {
            println!(
                "cargo:warning={env_gmsh} is set to {lib_dir:?}, but no shared library files were found there"
            );
        }
    }
    // 2. search system libraries
    //    -- try pkg-config on linux
    else {
        match try_pkgconfig() {
            Ok(link_paths) => {
                for path in link_paths.into_iter() {
                    println!("cargo:rustc-link-search={}", path.display());
                }
            }
            Err(not_found) => {
                eprintln!("pkg-config couldn't find Gmsh library {not_found}, ",)
            }
        }
    }

    // always link the gmsh shared library at the end
    println!("cargo:rustc-link-lib=gmsh");
}

fn try_pkgconfig() -> Result<Vec<PathBuf>, String> {
    let mut pkg = pkg_config::Config::new();
    pkg.atleast_version("4.4");

    match pkg.probe("gmsh") {
        Ok(gmsh_lib) => Ok(gmsh_lib.link_paths), // println!("found it"); true},
        Err(err) => Err(err.to_string()), // println!("cargo:warning={}, shared Gmsh library wasn't found. false},
    }
}
