# ffi-checker

0. (如果需要) 删除系统中已有的 LLVM

以 Ubuntu apt 安装的 llvm 为例
```bash
sudo apt list --installed | grep llvm
sudo apt remove ****
```

1. Download LLVM

https://github.com/llvm/llvm-project/releases/ 到这里下载最新的预编译包，
将 bin 目录加入的到 PATH 中

以 Ubuntu 和 LLVM19为例子

```bash
cd ~
wget https://github.com/llvm/llvm-project/releases/download/llvmorg-19.1.4/LLVM-19.1.4-Linux-X64.tar.xz
tar -xvf LLVM-19.1.4-Linux-X64.tar.xz
echo "export PATH="\$PATH:$HOME/LLVM-19.1.4-Linux-X64/bin"" >> .bashrc
source .bashrc
clang --version
```

此时应该可以看到 clang 的相关信息

2. Build ffi-checker
```bash
cd ~
git clone https://github.com/deltaLRD/ffi-checker.git
cd ffi-checker
cargo build --release
```

3. find a target project

这里以 iredismodule 为例

```bash
cd ~
git clone https://github.com/sigoden/iredismodule.git
cd iredismodule
~/ffi-checker/target/release/cargo-ffi-checker
```
等待结束就可以在 ~/iredismodule/target/debug/deps 里面看到 .bc 的 llvm bitcode 文件