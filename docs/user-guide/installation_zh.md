# 安装指南

本指南说明如何在您的系统上安装lawkit。

## 系统要求

- **操作系统**: Windows、macOS、Linux
- **Rust**: 1.70.0或更高版本（从Cargo安装时）
- **内存**: 最小512MB（处理大文件建议2GB+）

## 安装方法

### 1. 从Cargo安装（推荐）

```bash
# 安装最新版本
cargo install lawkit

# 安装特定版本
cargo install lawkit --version 2.0.0
```

### 2. 从源码构建

```bash
# 克隆仓库
git clone https://github.com/user/lawkit.git
cd lawkit

# 构建和安装
cargo build --release
cargo install --path .
```

### 3. 二进制下载

从[GitHub Releases](https://github.com/user/lawkit/releases)下载适合您平台的二进制文件。

#### Windows
```powershell
# 在PowerShell中运行
Invoke-WebRequest -Uri "https://github.com/user/lawkit/releases/latest/download/lawkit-windows.zip" -OutFile "lawkit.zip"
Expand-Archive lawkit.zip
```

#### macOS
```bash
# 使用Homebrew安装（计划中）
brew install lawkit

# 或手动下载
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-macos.tar.gz | tar xz
```

#### Linux
```bash
# 手动下载
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-linux.tar.gz | tar xz
sudo mv lawkit /usr/local/bin/
```

## 验证安装

确认安装成功：

```bash
# 检查版本
lawkit --version

# 显示帮助
lawkit --help

# 显示可用的统计法则
lawkit list
```

## 从旧版benf迁移

如果您正在使用现有的`benf`工具：

```bash
# 旧版用法
benf data.csv

# 新版等效用法
lawkit benf data.csv
```

保持100%兼容性，您可以原样使用现有脚本。

## 更新

### 如果通过Cargo安装
```bash
cargo install lawkit --force
```

### 如果通过二进制安装
下载并替换为新的二进制文件。

## 卸载

### 如果通过Cargo安装
```bash
cargo uninstall lawkit
```

### 如果手动安装
```bash
# 检查二进制位置
which lawkit

# 删除文件
sudo rm /usr/local/bin/lawkit
```

## 故障排除

### 编译错误
可能是由于Rust版本过旧：
```bash
rustup update stable
```

### 路径配置
如果二进制文件不在PATH中：
```bash
# 添加到~/.bashrc或~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
```

### 权限错误
如果在Linux上出现权限错误：
```bash
sudo chown $USER:$USER ~/.cargo/bin/lawkit
chmod +x ~/.cargo/bin/lawkit
```

## 下一步

安装完成后，在[快速开始](getting-started_zh.md)中学习基本用法。

- [快速开始](getting-started_zh.md) - 基本用法
- [使用示例](examples_zh.md) - 实际示例
- [CLI参考](../reference/cli-reference_zh.md) - 命令详细信息