# 安装指南

本指南将帮助您在系统上安装lawkit。

## 系统要求

- Rust 1.75或更高版本
- 支持的操作系统：Linux、macOS、Windows

## 安装方法

### 从Cargo安装（推荐）

```bash
cargo install lawkit
```

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/user/lawkit.git
cd lawkit

# 编译和安装
cargo build --release
cargo install --path lawkit-cli
```

### 使用预编译二进制文件

从[GitHub Releases](https://github.com/user/lawkit/releases)页面下载适合您系统的预编译二进制文件。

#### Linux/macOS

```bash
# 下载并安装
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-linux-x86_64.tar.gz | tar xz
sudo mv lawkit /usr/local/bin/
```

#### Windows

下载`lawkit-windows-x86_64.zip`，解压到合适的目录，并将该目录添加到PATH环境变量中。

## 验证安装

```bash
# 检查版本
lawkit --version

# 运行帮助命令
lawkit --help

# 列出可用的统计法则
lawkit list
```

## 故障排除

### 常见问题

#### Rust未安装
如果您没有安装Rust，请访问[rustup.rs](https://rustup.rs/)进行安装。

#### 权限错误
在Unix系统上，您可能需要使用`sudo`来安装到系统目录：

```bash
sudo cargo install lawkit
```

#### 网络问题
如果遇到网络连接问题，可以尝试配置代理或使用镜像源。

## 下一步

安装完成后，请阅读[入门指南](getting-started_zh.md)开始使用lawkit。

- [入门指南](getting-started_zh.md) - 学习基本功能
- [使用示例](examples_zh.md) - 实际应用场景
- [CLI参考文档](../reference/cli-reference_zh.md) - 命令详情