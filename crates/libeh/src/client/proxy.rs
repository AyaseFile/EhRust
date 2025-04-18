use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{env, fmt};

/// EhClient 代理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EhClientProxy {
    pub protocol: String,
    pub host: String,
    pub port: i32,
}

impl EhClientProxy {
    /// 创建一个新的 EhClientProxy 实例
    pub fn new(protocol: &str, host: &str, port: i32) -> Self {
        EhClientProxy {
            protocol: protocol.to_string(),
            host: host.to_string(),
            port,
        }
    }

    /// 从环境变量中获取代理设置
    pub fn env() -> Option<Self> {
        let url = if let Ok(url) = env::var("EH_PROXY") {
            url
        } else if let Ok(url) = env::var("HTTP_PROXY") {
            url
        } else {
            return None;
        };
        if let Ok(url) = Url::parse(&url) {
            let schema = url.scheme();
            let host = url.host_str().unwrap_or("localhost");
            let port = url.port().unwrap_or(1080);
            Some(EhClientProxy::new(schema, host, port.into()))
        } else {
            None
        }
    }
}

impl fmt::Display for EhClientProxy {
    /// 将 EhClientProxy 转换为 URL 字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}://{}:{}", self.protocol, self.host, self.port)
    }
}
