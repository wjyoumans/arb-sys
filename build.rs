//use std::{collections::VecDeque, ffi::OsString, fs};

use std::{
    env,
    ffi::{OsStr, OsString},
    fs::self,
    io::{Result as IoResult},
    path::{Path, PathBuf},
    process::Command,
    str,
};

const ARB_DIR: &str = "arb-c2b7e36-c";
const ARB_LIB: &str = "libarb.a";
const ARB_VER: &str = "c2b7e36";
const ARB_HEADERS: &[&str] = &[
    "acb_calc.h",
    "acb_dft.h",
    "acb_dirichlet.h",
    "acb_elliptic.h",
    "acb.h",
    "acb_hypgeom.h",
    "acb_mat.h",
    "acb_modular.h",
    "acb_poly.h",
    "arb_calc.h",
    "arb_fmpz_poly.h",
    "arb_fpwrap.h",
    "arb.h",
    "arb_hypgeom.h",
    "arb_mat.h",
    "arb_poly.h",
    "arf.h",
    "bernoulli.h",
    "bool_mat.h",
    "dirichlet.h",
    "dlog.h",
    "double_interval.h",
    "fmpr.h",
    "fmpz_extras.h",
    "hypgeom.h",
    "mag.h",
    "partitions.h",
];

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Mingw,
    Msvc,
    Other,
}

struct Environment {
    target: Target,
    gmp_mpfr_dir: PathBuf,
    flint_dir: PathBuf,
    src_dir: PathBuf,
    out_dir: PathBuf,
    lib_dir: PathBuf,
    include_dir: PathBuf,
    build_dir: PathBuf,
    cache_dir: Option<PathBuf>,
}

fn main() {
    let cc = env::var_os("CC");
    let cc_cache_dir = cc.as_ref().map(|cc| {
        let mut dir = OsString::from("CC-");
        dir.push(cc);
        dir
    });

    let raw_target = cargo_env("TARGET")
        .into_string()
        .expect("env var TARGET having sensible characters");

    let target = if raw_target.contains("-windows-msvc") {
        Target::Msvc
    } else if raw_target.contains("-windows-gnu") {
        Target::Mingw
    } else {
        Target::Other
    };

    let gmp_mpfr_dir = PathBuf::from(cargo_env("DEP_GMP_OUT_DIR"));
    let flint_dir = PathBuf::from(cargo_env("DEP_FLINT_OUT_DIR"));
    let src_dir = PathBuf::from(cargo_env("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(cargo_env("OUT_DIR"));

    println!("cargo:rerun-if-env-changed=ARB_SYS_CACHE");
    let cache_dir = if env::var("DOCS_RS").is_ok() {
         None
    } else {
        match env::var_os("ARB_SYS_CACHE") {
            Some(ref c) if c.is_empty() || c == "_" => None,
            Some(c) => Some(PathBuf::from(c)),
            None => system_cache_dir().map(|c| c.join("arb-sys")),
        }
    };
    let cache_dir = cache_dir
        .map(|cache| cache.join(&ARB_VER))
        .map(|cache| match cc_cache_dir {
            Some(dir) => cache.join(dir),
            None => cache,
        });

    let env = Environment {
        target,
        gmp_mpfr_dir,
        flint_dir,
        src_dir,
        out_dir: out_dir.clone(),
        lib_dir: out_dir.join("lib"),
        include_dir: out_dir.join("include"),
        build_dir: out_dir.join("build"),
        cache_dir,
    };
       
    // make sure we have target directories
    create_dir_or_panic(&env.lib_dir);
    create_dir_or_panic(&env.include_dir);

    compile(&env);
}

fn compile(env: &Environment) {
    if need_compile(env) {
        check_for_msvc(env);
        remove_dir_or_panic(&env.build_dir);
        copy_dir_or_panic(&env.src_dir.join(ARB_DIR), &env.build_dir);
        build(env);
        save_cache(env);
    }
    write_link_info(env);
}

fn need_compile(env: &Environment) -> bool {
    let mut ok = env.lib_dir.join(ARB_LIB).is_file();  
    for h in ARB_HEADERS {
        ok = ok && env.include_dir.join(h).is_file();
    }

    if ok {
        if should_save_cache(env) {
            save_cache(env);
        }
        return false;
    } else if load_cache(env) {
        // if loading cache works, we're done
        return false;
    }
    true
}

fn save_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = create_dir(&cache_dir).is_ok();
    ok = ok && copy_file(&env.lib_dir.join(ARB_LIB), &cache_dir.join(ARB_LIB)).is_ok();

    for h in ARB_HEADERS {
        ok = ok && copy_file(&env.include_dir.join(h), &cache_dir.join(h)).is_ok();
    }
    ok
}

fn load_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = true;
    ok = ok && copy_file(&cache_dir.join(ARB_LIB), &env.lib_dir.join(ARB_LIB)).is_ok();

    for h in ARB_HEADERS {
        ok = ok && copy_file(&cache_dir.join(h), &env.include_dir.join(h)).is_ok();
    }
    ok
}

fn should_save_cache(env: &Environment) -> bool {
    let cache_dir = match env.cache_dir {
        Some(ref s) => s,
        None => return false,
    };
    let mut ok = true;
    ok = ok && cache_dir.join(ARB_LIB).is_file();

    for h in ARB_HEADERS {
        ok = ok && cache_dir.join(h).is_file();
    }
    !ok
}

fn build(env: &Environment) {
    println!("$ cd {:?}", &env.build_dir);
    let conf = String::from(
        format!(
            "./configure --disable-shared --with-gmp={} --with-flint={}",
            env.gmp_mpfr_dir.display(),
            env.flint_dir.display(),
            )
        );

    configure(&env.build_dir, &OsString::from(conf));
    make_and_check(env, &env.build_dir);

    let build_lib = &env.build_dir.join(ARB_LIB);
    copy_file_or_panic(&build_lib, &env.lib_dir.join(ARB_LIB));
    
    for h in ARB_HEADERS {
        copy_file_or_panic(&env.build_dir.join(h), &env.include_dir.join(h));
    }
}

fn write_link_info(env: &Environment) {
    let out_str = env.out_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.out_dir.display()
        )
    });
    let lib_str = env.lib_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.lib_dir.display()
        )
    });
    let include_str = env.include_dir.to_str().unwrap_or_else(|| {
        panic!(
            "Path contains unsupported characters, can only make {}",
            env.include_dir.display()
        )
    });

    println!("cargo:out_dir={}", out_str);
    println!("cargo:lib_dir={}", lib_str);
    println!("cargo:include_dir={}", include_str);
    println!("cargo:rustc-link-search=native={}", lib_str);
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=mpfr");
    println!("cargo:rustc-link-lib=static=flint");
    println!("cargo:rustc-link-lib=static=arb");
}


fn cargo_env(name: &str) -> OsString {
    env::var_os(name)
        .unwrap_or_else(|| panic!("environment variable not found: {}, please use cargo", name))
}

fn check_for_msvc(env: &Environment) {
    if env.target == Target::Msvc {
        panic!("Windows MSVC target is not supported (linking would fail)");
    }
}

fn remove_dir(dir: &Path) -> IoResult<()> {
    if !dir.exists() {
        return Ok(());
    }
    assert!(dir.is_dir(), "Not a directory: {:?}", dir);
    println!("$ rm -r {:?}", dir);
    fs::remove_dir_all(dir)
}

fn remove_dir_or_panic(dir: &Path) {
    remove_dir(dir).unwrap_or_else(|_| panic!("Unable to remove directory: {:?}", dir));
}

fn create_dir(dir: &Path) -> IoResult<()> {
    println!("$ mkdir -p {:?}", dir);
    fs::create_dir_all(dir)
}

fn create_dir_or_panic(dir: &Path) {
    create_dir(dir).unwrap_or_else(|_| panic!("Unable to create directory: {:?}", dir));
}

pub fn copy_dir(from: &Path, to: &Path) -> IoResult<()> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from));

    let output_root = PathBuf::from(to);
    let input_root = PathBuf::from(from).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!("$ mkdir {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn copy_dir_or_panic(src: &Path, dst: &Path) {
    copy_dir(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn copy_file(src: &Path, dst: &Path) -> IoResult<u64> {
    println!("$ cp {:?} {:?}", src, dst);
    fs::copy(src, dst)
}

fn copy_file_or_panic(src: &Path, dst: &Path) {
    copy_file(src, dst).unwrap_or_else(|_| {
        panic!("Unable to copy {:?} -> {:?}", src, dst);
    });
}

fn configure(build_dir: &Path, conf_line: &OsStr) {
    let mut conf = Command::new("sh");
    conf.current_dir(&build_dir).arg("-c").arg(conf_line);
    execute(conf);
}

fn make_and_check(_env: &Environment, build_dir: &Path) {
    let mut make = Command::new("make");
    make.current_dir(build_dir);
    execute(make);

    if !cfg!(feature = "disable-make-check") {
        let mut make_check = Command::new("make");
        make_check
            .current_dir(build_dir)
            .arg("check");
        execute(make_check);
    }
}

fn execute(mut command: Command) {
    println!("$ {:?}", command);
    let status = command
        .status()
        .unwrap_or_else(|_| panic!("Unable to execute: {:?}", command));
    if !status.success() {
        if let Some(code) = status.code() {
            panic!("Program failed with code {}: {:?}", code, command);
        } else {
            panic!("Program failed: {:?}", command);
        }
    }
}

fn system_cache_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        use core::{mem::MaybeUninit, ptr, slice};
        use std::os::windows::ffi::OsStringExt;
        use winapi::{
            shared::winerror::S_OK,
            um::{combaseapi, knownfolders::FOLDERID_LocalAppData, shlobj, winbase},
        };
        let id = &FOLDERID_LocalAppData;
        let flags = 0;
        let access = ptr::null_mut();
        let mut path = MaybeUninit::uninit();
        unsafe {
            if shlobj::SHGetKnownFolderPath(id, flags, access, path.as_mut_ptr()) == S_OK {
                let path = path.assume_init();
                let slice = slice::from_raw_parts(path, winbase::lstrlenW(path) as usize);
                let string = OsString::from_wide(slice);
                combaseapi::CoTaskMemFree(path as _);
                Some(string.into())
            } else {
                None
            }
        }
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        env::var_os("HOME")
            .filter(|x| !x.is_empty())
            .map(|x| AsRef::<Path>::as_ref(&x).join("Library").join("Caches"))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "ios")))]
    {
        env::var_os("XDG_CACHE_HOME")
            .filter(|x| !x.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME")
                    .filter(|x| !x.is_empty())
                    .map(|x| AsRef::<Path>::as_ref(&x).join(".cache"))
            })
    }
}

