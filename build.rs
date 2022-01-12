use std::{collections::VecDeque, ffi::OsString, fs};

fn main() {
    // Possible libraries to link to, depending on the system.
    let arb_libs = ["arb", "flint-arb"];
    // Directories in which to look (recursively) for the library.
    let lib_dirs = ["/lib", "/usr/lib"];

    if let Some(l) = find(&arb_libs, &lib_dirs) {
        println!("cargo:rustc-link-lib={}", l);
    } else {
        panic!("Arb library not detected!");
    }
}

macro_rules! skip_if_err {
    ($e: expr) => { match $e {
        Ok(v) => v,
        Err(_) => continue
    }}
}

fn find<'a>(libs: &'a [&str], dirs: &[&str]) -> Option<&'a str> {
    let libs: Vec<(&str,String)> =
        libs.iter().map(|&l| (l, format!("lib{}.so", l))).collect();
    for d0 in dirs.iter() {
        let mut q = VecDeque::new();  // dirs to examine
        q.push_back(OsString::from(d0));
        while let Some(d) = q.pop_front() {
            let entries = skip_if_err!(fs::read_dir(d));
            for entry in entries {
                let entry = skip_if_err!(entry);
                if skip_if_err!(entry.file_type()).is_dir() {
                    q.push_back(entry.path().into_os_string());
                    continue
                }
                let name = skip_if_err!(entry.file_name().into_string());
                for (lib, fname) in &libs {
                    if name == *fname {
                        return Some(lib);
                    }
                }
            }
        }
    }
    None
}
