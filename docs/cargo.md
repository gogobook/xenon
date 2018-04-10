# Cargo

## Cargo 命令行选项

`cargo <command> [<args>...]`  

命令行选项:
    -h, --help          显示帮助
    -V, --version       打印版本信息之后退出
    --list              列出支持的命令
    --explain CODE      运行 `rustc --explain CODE`
    -v, --verbose ...   显示详细(冗长)输出 (-vv very verbose/build.rs output)
    -q, --quiet         静默输出
    --color WHEN        输出着色: auto, always, never
    --frozen            Require Cargo.lock and cache are up to date
    --locked            Require Cargo.lock is up to date
    -Z FLAG ...         Unstable (nightly-only) flags to Cargo

|命令|解释|
|:---|:---|
|bench       | 运行基准测试|
|build       | 编译当前项目|
|check       | 分析当前项目, 报告错误, 但是不会构建二进制文件 |
|clean       | 移除 `target` 目录 |
|doc         | 构建项目和依赖的项目的文档 |
|fetch | |
|fmt | |
|generate-lockfile | |
|git-checkout |  |
|help | |
|init        | 在已存在的目录中创建 `cargo` 项目 |
|install     | 安装 Rust 二进制|
|locate-project | |
|login | |
|metadata | |
|new         | 创建 `cargo` 项目 |
|owner | |
|package | |
|pkgid |  |
|publish     | 打包上传当前项目到 registry|
|read-manifest | |
|run         | 构建执行 `src/main.rs` |
|rustc | |
|rustdoc | |
|search      | 搜索 注册过的 `crates`|
|test        | 运行测试 |
|uninstall   | 卸载 Rust 二进制|
|update      | 更新 `Cargo.lock` 中存在的依赖项 |
|verify-project | |
|version | |
|yank | |