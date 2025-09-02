// Remove this deprecated import
// use bindgen::CargoCallbacks;
// 构建脚本需要放在项目根目录
fn main() {
    // 指定需要链接的库
    println!("cargo:rustc-link-lib=rknn_api");
    
    // 设置多个头文件路径
    let headers = [
        "libs/librknn_api/include/rknn_api.h",
        "libs/librknn_api/include/rknn_matmul_api.h",
        "libs/librknn_api/include/rknn_custom_op.h"
    ];
    let builder = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // 添加每个头文件及其包含路径
    let builder = headers.iter().fold(builder, |b, h| {
        b.header(*h)
         .clang_arg(format!("-I{}", std::path::Path::new(h).parent().unwrap().display()))
    });

    let bindings = builder
        .allowlist_type("rknn_.*")
        .allowlist_function("rknn_.*")
        .allowlist_var("RKNN_.*")
        .derive_default(true)
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
