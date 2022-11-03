const CMAKE_BUILD_TYPE: &str = "Debug";

fn main() {
    let dst = cmake::Config::new("aws-c-common").define("CMAKE_BUILD_TYPE", CMAKE_BUILD_TYPE).build();
    eprintln!("dst={}", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=aws-c-common");
}

// #[derive(Debug)]
// enum BuildError {
//     Io(IoError),
//     Var(VarError),
// }

// impl From<IoError> for BuildError {
//     fn from(e: IoError) -> Self {
//         BuildError::Io(e)
//     }
// }

// impl From<VarError> for BuildError {
//     fn from(e: VarError) -> Self {
//         BuildError::Var(e)
//     }
// }

// #[cfg(feature = "nocmake")]
// fn main() -> Result<(), BuildError> {
//     let mut builder = cc::Build::new();

//     let root = var("CARGO_MANIFEST_DIR")?;
//     builder.include(format!("{root}/aws-c-common/include"));

//     // Generate the aws/common/config.h file.
//     create_dir_all(format!("{root}/generated/include/aws/common"))?;
//     let mut config = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .open(format!("{root}/generated/include/aws/common/config.h"))
//         .unwrap();

//     let mut aws_have_gcc_overflow_math_extensions = "/* #undef AWS_HAVE_GCC_OVERFLOW_MATH_EXTENSIONS */";
//     let mut aws_have_gcc_inline_asm = "/* #undef AWS_HAVE_GCC_INLINE_ASM */";
//     let mut aws_have_msvc_mulx = "/* #undef AWS_HAVE_MSVC_MULX */";
//     let mut aws_have_posix_large_file_support = "/* #undef AWS_HAVE_POSIX_LARGE_FILE_SUPPORT */";
//     let mut aws_have_execinfo = "/* #undef AWS_HAVE_EXECINFO */";
//     let mut aws_have_winapi_desktop = "/* #undef AWS_HAVE_WINAPI_DESKTOP */";
//     let mut aws_have_linux_if_link_h = "/* #undef AWS_HAVE_LINUX_IF_LINK_H */";

//     if cfg!(unix) {
//         aws_have_gcc_overflow_math_extensions = "#define AWS_HAVE_GCC_OVERFLOW_MATH_EXTENSIONS";
//         aws_have_gcc_inline_asm = "#define AWS_HAVE_GCC_INLINE_ASM";
//         aws_have_posix_large_file_support = "#define AWS_HAVE_POSIX_LARGE_FILE_SUPPORT";
//         aws_have_execinfo = "#define AWS_HAVE_EXECINFO";

//         if cfg!(target_os = "linux") {
//             aws_have_linux_if_link_h = "#define AWS_HAVE_LINUX_IF_LINK_H";
//         }
//     } else if cfg!(windows) {
//         aws_have_msvc_mulx = "#define AWS_HAVE_MSVC_MULX";
//         aws_have_winapi_desktop = "#define AWS_HAVE_WINAPI_DESKTOP";
//     }

//     write!(
//         config,
// r#"#ifndef AWS_COMMON_CONFIG_H
// #define AWS_COMMON_CONFIG_H

// {aws_have_gcc_overflow_math_extensions}
// {aws_have_gcc_inline_asm}
// {aws_have_msvc_mulx}
// {aws_have_posix_large_file_support}
// {aws_have_execinfo}
// {aws_have_winapi_desktop}
// {aws_have_linux_if_link_h}

// #endif
// "#)?;
//     config.flush()?;
//     drop(config);

//     let mut directories = vec![format!("aws-c-common/source"), format!("aws-c-common/source/external")];

//     if cfg!(unix) {
//         directories.push(format!("aws-c-common/source/posix"));
//     }

//     if cfg!(windows) {
//         directories.push(format!("aws-c-common/source/windows"));
//     }

//     for dir in directories {
//         let entries = match read_dir(dir) {
//             Ok(entries) => entries,
//             Err(e) => panic!("Failed to read directory: {}", e),
//         };

//         for maybe_entry in entries {
//             let entry = match maybe_entry {
//                 Ok(entry) => entry,
//                 Err(e) => panic!("Failed to read entry: {}", e),
//             };

//             let file_name = entry.file_name();
//             let file_name_bytes = file_name.as_bytes();
//             let file_name_len = file_name_bytes.len();
//             if file_name_len > 2 && &file_name_bytes[(file_name_len - 2)..file_name_len] == b".c" {
//                 builder.file(entry.path());
//             }
//         }
//     }

//     if cfg!(target_arch = "aarch64") && (cfg!(target_os = "linux") || cfg!(target_os = "freebsd")) {
//         builder.file(format!("aws-c-common/source/arch/arm/asm/cpuid.c"));
//     } else if cfg!(target_arch = "aarch64") && cfg!(target_os = "windows") {
//         builder.file(format!("aws-c-common/source/arch/arm/msvc/cpuid.c"));
//     } else if cfg!(target_arch = "x86_64") {
//         builder.file(format!("aws-c-common/source/arch/intel/cpuid.c"));
//         builder.file(format!("aws-c-common/source/arch/intel/encoding_avx2.c"));

//         if cfg!(windows) {
//             builder.file(format!("aws-c-common/source/arch/intel/msvc/cpuid.c"));
//         } else {
//             builder.file(format!("aws-c-common/source/arch/intel/asm/cpuid.c"));
//         }
//     } else {
//         builder.file(format!("aws-c-common/source/arch/generic/cpuid.c"));
//     }

//     builder.define("AWS_AFFINITY_METHOD", "AWS_AFFINITY_METHOD_NONE");
//     builder.define("CJSON_HIDE_SYMBOLS", None);
//     builder.define("DEBUG_BUILD", None);
    
//     if cfg!(unix) {
//         builder.define("HAVE_SYSCONF", None);
//     }

//     builder.compile("aws-c-common");
//     Ok(())
// }
