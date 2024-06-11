use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

fn main() {
    // 创建一个服务器实例，参数为None表示使用默认配置
    let mut my_server = Server::new(None).unwrap();
    // 初始化服务器
    my_server.bootstrap();
    // 创建一个负载均衡器，包含两个上游服务器
    let upstreams = LoadBalancer::try_from_iter(["192.168.9.34:80", "10.0.0.9:80"]).unwrap();
    // 创建一个HTTP代理服务，并传入服务器配置和负载均衡器
    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
    // 添加一个TCP监听地址
    lb.add_tcp("0.0.0.0:6188");
    // 将服务添加到服务器中
    my_server.add_service(lb);
    // 运行服务器，进入事件循环
    my_server.run_forever();
}

// 定义一个包含负载均衡器的结构体LB，用于包装Arc指针以实现多线程共享。
pub struct LB(Arc<LoadBalancer<RoundRobin>>);

// 使用#[async_trait]宏，异步实现ProxyHttp trait。
#[async_trait]
impl ProxyHttp for LB {
    /// 定义上下文类型，这里使用空元组。对于这个小例子，我们不需要上下文存储
    type CTX = ();
    // 创建新的上下文实例，这里返回空元组
    fn new_ctx(&self) -> () {
        ()
    }
    // 选择上游服务器并创建HTTP对等体
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        // 使用轮询算法选择上游服务器
        let upstream = self
            .0
            .select(b"", 256) // 对于轮询，哈希不重要
            .unwrap();
        println!("上游对等体是：{upstream:?}");
        // 创建一个新的HTTP对等体，设置SNI为example.com
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(upstream, false, "example.com".to_string()));
        Ok(peer)
    }

    // 在上游请求发送前，插入Host头部
    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        // 将Host头部设置为example.com
        upstream_request
            .insert_header("Host", "example.com")
            .unwrap();
        Ok(())
    }
}

/* 备注
在TLS（Transport Layer Security）和SSL（Secure Sockets Layer）协议中，
SNI（Server Name Indication）是一个扩展字段，用于指定客户端要连接的目标服务器的主机名。

当客户端发起TLS握手时，它会发送一个加密的客户端Hello消息给服务器，其中包含SNI字段，告诉服务器它希望连接的是哪个主机名。
服务器可以根据SNI字段中的主机名来选择相应的证书，从而支持在同一IP地址上托管多个域名的HTTPS站点。

在HTTP代理服务器中，如果代理服务器需要与上游服务器建立加密连接（HTTPS），则通常需要在代理请求中包含SNI字段，以确保上游服务器能够正确识别客户端请求的主机名。
因此，在创建新的HTTP对等体时，设置SNI为example.com意味着在与上游服务器建立加密连接时，客户端会指定目标主机名为example.com
 */