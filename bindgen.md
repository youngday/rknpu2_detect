在 Rust 项目中正确使用 bindgen 需要以下两个步骤：

系统级依赖安装（以 Ubuntu/Debian 为例）：

bash
sudo apt-get update
sudo apt-get install clang libclang-dev llvm-dev
确认 Cargo.toml 中已正确配置（根据你提供的文件，已包含）：

Cargo.toml
Apply
[build-dependencies]
cc = "1.2"
bindgen = "0.72"  # 已存在，版本正确
常见问题处理：

如果遇到链接错误，尝试设置环境变量：

bash
export LIBCLANG_PATH=/usr/lib/llvm-<version>/lib
安装完成后需要 clean 构建：

bash
cargo clean && cargo build