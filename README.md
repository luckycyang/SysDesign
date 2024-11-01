## 简易说明

机器学习框架为 [`burn`](https://burn.dev/)

GUI框架为 [`slint`](https://slint.dev/)


## 运行

确保安装 `rust` 开发环境

`linux` 使用如下命令安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`windows` 通过下载运行 [`rustup-init.exe`](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) 安装


确保 `rust` 组件完备以及版本

```bash
rustup update
```

## 界面

左上角选择模型的权重文件, 加载权重文件后方可推理

![image.png](https://s2.loli.net/2024/10/31/D7LbBkZRQzv3ESX.png)

点击选择一张图片即可返回结果， 如果图片过大， 由于显存不足而导致程序崩溃

![image.png](https://s2.loli.net/2024/10/31/z5huVHx7ag9pAsv.png)