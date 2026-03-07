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

# ===== 预编译项目依赖（加速 CI 构建） =====
# 使用固定的 target 目录，CI 构建时复用预编译产物
ENV CARGO_TARGET_DIR=/cargo-target

# 复制项目依赖声明文件
WORKDIR /tmp/deps
COPY Cargo.toml Cargo.lock build.rs ./
COPY crates/cnb-api/Cargo.toml crates/cnb-api/
COPY crates/cnb-core/Cargo.toml crates/cnb-core/
COPY crates/cnb-tui/Cargo.toml crates/cnb-tui/
COPY crates/cnb-chat/Cargo.toml crates/cnb-chat/build.rs crates/cnb-chat/

# 创建空源文件和必要目录（仅用于依赖解析和预编译）
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs \
    && mkdir -p crates/cnb-api/src && touch crates/cnb-api/src/lib.rs \
    && mkdir -p crates/cnb-core/src && touch crates/cnb-core/src/lib.rs \
    && mkdir -p crates/cnb-tui/src && touch crates/cnb-tui/src/lib.rs \
    && mkdir -p crates/cnb-chat/src && touch crates/cnb-chat/src/lib.rs \
    && mkdir -p crates/cnb-chat/references

# 预编译所有 target 的依赖（原生构建）
RUN cargo build --release --target x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-pc-windows-gnu

# 预编译所有 target 的依赖（zigbuild 交叉编译）
RUN cargo zigbuild --release --target x86_64-unknown-linux-musl
RUN cargo zigbuild --release --target aarch64-unknown-linux-gnu
RUN cargo zigbuild --release --target aarch64-unknown-linux-musl

# macOS 交叉编译需要禁用 jitterentropy（缺少 CoreServices 框架头文件）
ENV AWS_LC_SYS_NO_JITTER_ENTROPY=1
RUN cargo zigbuild --release --target x86_64-apple-darwin
# aarch64-apple-darwin: Zig 交叉编译器不会自动定义 ARM NEON/Crypto 宏，
# 但 Apple Silicon 架构必定支持这些扩展，手动定义以通过 aws-lc-sys 编译检查
ENV CFLAGS_aarch64_apple_darwin="-D__ARM_NEON=1 -D__ARM_FEATURE_CRYPTO=1"
RUN cargo zigbuild --release --target aarch64-apple-darwin

# 清理工作区 crate 的编译产物（保留外部依赖的缓存）
# 预编译使用空源文件，生成的 workspace crate 产物不包含实际符号，
# 必须删除以确保 CI 构建时 cargo 重新编译项目自身的 crate
RUN find /cargo-target -type d \( \
      -name "cnb-*" -o -name "cnb_*" -o -name "git-cnb-*" \
    \) -exec rm -rf {} + 2>/dev/null; \
    find /cargo-target -type f \( \
      -name "cnb-*" -o -name "cnb_*" -o -name "libcnb*" -o -name "git-cnb*" \
    \) -delete 2>/dev/null; \
    true

# 清理临时项目文件
RUN rm -rf /tmp/deps
WORKDIR /
