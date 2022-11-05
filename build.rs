use {
    log::{error, info},
    std::{env::var, fs::create_dir_all, process::Command},
};

const CMAKE_BUILD_TYPE: &str = "Debug";
const LIBRARIES: [&str; 10] = [
    "aws-c-common",
    "aws-lc",
    "s2n-tls",
    "aws-c-cal",
    "aws-c-io",
    "aws-c-io",
    "aws-c-compression",
    "aws-c-http",
    "aws-c-sdkutils",
    "aws-c-auth",
];

fn main() {
    let out_dir = var("OUT_DIR").unwrap();
    env_logger::init();

    println!("cargo:rustc-link-search=native={out_dir}/lib");
    println!("cargo:rerun-if-changed=build.rs");

    for library in LIBRARIES {
        // Create the CMake build directory.
        let lib_cmake_dir = format!("{out_dir}/cmake/{library}/build");
        create_dir_all(&lib_cmake_dir).unwrap();

        // Prepare the build.
        let args = vec![
            "-S".to_string(),
            library.to_string(),
            "-B".to_string(),
            lib_cmake_dir.clone(),
            format!("-DCMAKE_BUILD_TYPE={CMAKE_BUILD_TYPE}"),
            format!("-DCMAKE_INSTALL_PREFIX={out_dir}"),
            format!("-DCMAKE_PREFIX_PATH={out_dir}"),
        ];
        info!("Preparing build for {library}: cmake {}", args.join(" "));

        match Command::new("cmake").args(args).status() {
            Ok(es) => {
                if !es.success() {
                    error!("cmake preparation for {} failed: {}", library, es);
                    panic!("cmake preparation for {} failed: {}", library, es);
                }
            }
            Err(e) => {
                error!("cmake preparation for {} failed: {}", library, e);
                panic!("cmake preparation for {} failed: {}", library, e);
            }
        }

        // Build the library.
        let args =
            vec!["--build".to_string(), lib_cmake_dir, "--target".to_string(), "install".to_string()];
        info!("Building {library}: cmake {}", args.join(" "));

        match Command::new("cmake").args(args).status() {
            Ok(es) => {
                if !es.success() {
                    error!("cmake build for {} failed: {}", library, es);
                    panic!("cmake build for {} failed: {}", library, es);
                }
            }
            Err(e) => {
                error!("cmake build for {} failed: {}", library, e);
                panic!("cmake build for {} failed: {}", library, e);
            }
        }

        match library {
            "aws-lc" => {
                println!("cargo:rustc-link-lib=static=crypto");
                println!("cargo:rustc-link-lib=static=ssl");
            }
            "s2n-tls" => {
                println!("cargo:rustc-link-lib=static=s2n");
            }
            _ => {
                println!("cargo:rustc-link-lib=static={}", library);
            }
        }
    }
}
