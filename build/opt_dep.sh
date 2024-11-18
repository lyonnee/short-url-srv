#!/bin/bash

# 检查并安装所需工具
install_tool() {
    local tool=$1
    if ! cargo install --list | grep -q "$tool"; then
        echo "install $tool ..."
        cargo install "$tool"
    else
        echo "$tool have installed"
fi
}

echo "Check to make sure any new hardware or software is properly installed..."
install_tool cargo-udeps
install_tool cargo-sort
install_tool cargo-outdated

echo "Switch to the nightly toolchain to support cargo-udeps ..."
rustup override set nightly

# 识别并移除未使用的crate
echo "Check the unused crate..."
cargo +nightly udeps || {
    echo "Failed to detect unused crate, check nightly version or project dependency"
}

# 排序并格式化Cargo.toml中的依赖
echo "Sorting and Formatting Cargo.toml Dependencies..."
cargo sort || {
    echo "Cargo.toml sort failed"
}

# 检查并列出过时的依赖项
echo "Check for outdated dependencies..."
cargo outdated || {
    echo "Unable to list outdated dependencies, check whether cargo-outdated is installed correctly"
}

# 恢复为默认工具链
echo "Restore to stable toolchain..."
rustup override unset

# 格式化代码
echo "Formatting rust code..."
rustup component add rustfmt
cargo fmt

echo "Optimize dependency management tasks!"