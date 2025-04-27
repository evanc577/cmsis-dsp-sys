use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // Modify CMSIS Makefile target and cpu
    let mcpu_re = regex::Regex::new(r"__MCPU__").unwrap();
    let target_re = regex::Regex::new(r"__TARGET__").unwrap();
    let cmsis_root_re = regex::Regex::new(r"__CMSIS_ROOT__").unwrap();
    let makefile = std::fs::read_to_string("Makefile").unwrap();
    let makefile = mcpu_re.replace_all(&makefile, env!("CMSIS_CPU"));
    let makefile = target_re.replace_all(&makefile, std::env::var("TARGET").unwrap());
    let makefile = cmsis_root_re.replace_all(&makefile, out_dir.to_str().unwrap());
    let makefile_path = out_dir.join("Makefile");
    std::fs::write(&makefile_path, makefile.as_bytes()).unwrap();

    // Download CMSIS sources
    download_sources();

    // Build CMSIS
    build(makefile_path);

    // Tell Cargo where the library is
    let libdir_path = out_dir
        .join("builddir")
        // Canonicalize the path as `rustc-link-search` requires an absolute path.
        .canonicalize()
        .expect("cannot canonicalize path");
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=CMSISDSP");

    // Ignore these macros because they generate duplicate definitions
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
        ]
        .into_iter()
        .collect(),
    );

    // Generate rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .clang_arg(format!("-I{}", "/usr/include"))
        .clang_arg(format!(
            "-I{}",
            out_dir.join("CMSIS_DSP/Include").to_str().unwrap()
        ))
        .clang_arg(format!(
            "-I{}",
            out_dir
                .join("CMSIS_CORE/CMSIS/Core/Include")
                .to_str()
                .unwrap()
        ))
        .ctypes_prefix("cty")
        .blocklist_function("^(__.*)$")
        .parse_callbacks(Box::new(ignored_macros))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn download_sources() {
    download_and_unzip(
        "https://github.com/ARM-software/CMSIS-DSP/releases/download/v1.16.2/ARM.CMSIS-DSP.1.16.2.pack",
        "CMSIS_DSP",
    );
    download_and_unzip(
        "https://github.com/ARM-software/CMSIS_6/releases/download/v6.1.0/ARM.CMSIS.6.1.0.pack",
        "CMSIS_CORE",
    );
}

fn download_and_unzip(url: &str, output_dir: impl AsRef<Path>) {
    let mut data = std::io::Cursor::new(reqwest::blocking::get(url).unwrap().bytes().unwrap());
    let mut archive = zip::ZipArchive::new(&mut data).unwrap();
    let output_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join(output_dir.as_ref());
    archive.extract(output_dir).unwrap();
}

fn build(makefile: impl AsRef<Path>) {
    std::process::Command::new("make")
        .args([
            "-f",
            makefile.as_ref().to_str().unwrap(),
            &format!("-j{}", std::thread::available_parallelism().unwrap()),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
