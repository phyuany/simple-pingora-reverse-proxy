# Pingora 实现的高效负载均衡 HTTP 代理

这个项目展示了如何使用 Rust 的 `pingora` crate 创建带负载均衡的 HTTP 代理服务器。

## 一、功能

- **HTTP 代理服务**：将传入的 HTTP 请求代理到上游服务器。
- **负载均衡**：使用轮询算法将流量分配到多个上游服务器。
- **TLS 支持**：使用指定的证书和密钥文件支持 TLS 和 HTTP/2。

## 二、入门

### 先决条件

- 已安装 Rust 编程语言。你可以从 [这里](https://www.rust-lang.org/) 安装。

### 2.1 项目结构

- `src/main.rs`：包含服务器设置和配置的主要 Rust 源文件。
- `keys/`：存放 TLS 证书 (`example.com.crt`) 和密钥 (`example.com.key`) 的目录。

### 2.2 设置

1. 克隆仓库：

    ```sh
    git clone https://github.com/phyuany/simple-pingora-reverse-proxy.git
    cd simple-pingora-reverse-proxy
    ```

2. 将你的 TLS 证书和密钥文件放入 `keys` 目录：

    ```sh
    mkdir keys
    # 将你的 example.com.crt 和 example.com.key 文件放到 keys 目录中
    ```

3. 构建并运行服务器：

    ```sh
    cargo run
    ```

### 2.3 服务器配置

- 服务器监听 `80` 端口处理 HTTP 流量。
- 服务器监听 `443` 端口处理带 TLS 的 HTTPS 流量。
- 使用轮询算法在两个上游服务器 `10.0.0.1:8080` 和 `10.0.0.2:8080` 之间进行负载均衡。

### 2.4 代码概览

- **服务器初始化**：创建并配置一个 `Server` 实例。
- **负载均衡器**：创建一个 `LoadBalancer` 实例来管理多个上游服务器。
- **HTTP 代理服务**：使用服务器设置和负载均衡器进行配置，并监听特定端口。
- **TLS 设置**：配置以启用 HTTP/2 并使用指定的证书和密钥文件。
- **服务列表**：将服务添加到服务器，服务器进入事件循环开始处理请求。

### 2.5 示例用法

该代理服务器设计用于将传入的 HTTP 请求转发到指定的上游服务器，使用轮询算法，并且为所有上游请求设置 `Host` 头为 `example.com`。

### 2.6 备注

- **SNI（服务器名称指示）**：`HttpPeer` 配置了 SNI，以确保在与上游服务器建立 TLS 连接时使用正确的主机名。

## 三、贡献

如果你有任何建议或改进，欢迎提交问题或拉取请求。

## 四、许可证

本项目使用 MIT 许可证。
