FROM rust:latest

# 安装系统依赖
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc-mingw-w64-x86-64 \
    zip \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# 安装 Zig（用于 cargo-zigbuild 交叉编译 macOS / musl / aarch64）
RUN curl -sSfL "https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz" \
    | tar -xJ -C /opt \
    && mv /opt/zig-linux-x86_64-0.13.0 /opt/zig
ENV PATH="/opt/zig:${PATH}"

# 安装 cargo-binstall（用于快速安装预编译的 Rust CLI 工具）
RUN curl -sSfL https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# 安装 cargo-zigbuild 和 git-cliff
RUN cargo binstall --no-confirm cargo-zigbuild git-cliff

# 配置 rustup 使用 tuna 镜像源（加速下载工具链组件）
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV RUSTUP_UPDATE_ROOT=https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup

# 添加所有交叉编译 targets
RUN rustup target add \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-gnu \
    aarch64-unknown-linux-musl \
    x86_64-pc-windows-gnu \
    x86_64-apple-darwin \
    aarch64-apple-darwin

# 配置 cargo 镜像源（国内加速）和 Windows 交叉编译链接器
RUN printf '\
[source.crates-io]\n\
replace-with = "mirror"\n\
\n\
[source.mirror]\n\
registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"\n\
\n\
[registries.mirror]\n\
index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"\n\
\n\
[net]\n\
git-fetch-with-cli = true\n\
\n\
[target.x86_64-pc-windows-gnu]\n\
linker = "x86_64-w64-mingw32-gcc"\n\
' >> ${CARGO_HOME}/config.toml
