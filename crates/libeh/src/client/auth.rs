use std::{env, fmt};

use serde::{Deserialize, Serialize};

/// E-Hentai/ExHentai 用户身份验证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EhClientAuth {
    /// E-Hentai/ExHentai 用户 ID
    pub ipb_member_id: String,
    /// E-Hentai/ExHentai 用户令牌
    pub ipb_pass_hash: String,
    /// ExHentai 访问令牌，为 None 时无 ExHentai 访问权限
    pub igneous: Option<String>,
}

impl EhClientAuth {
    /// 创建一个新的 EhClientAuth 实例
    pub fn new(ipb_member_id: &str, ipb_pass_hash: &str, igneous: Option<&str>) -> Self {
        EhClientAuth {
            ipb_member_id: ipb_member_id.to_string(),
            ipb_pass_hash: ipb_pass_hash.to_string(),
            igneous: igneous.map(|s| s.to_string()),
        }
    }

    /// 从环境变量中读取 EhClientAuth 信息
    pub fn env() -> Option<Self> {
        let ipb_member_id = if let Ok(ipb_member_id) = env::var("EH_AUTH_ID") {
            ipb_member_id
        } else {
            return None;
        };
        let ipb_pass_hash = if let Ok(ipb_pass_hash) = env::var("EH_AUTH_HASH") {
            ipb_pass_hash
        } else {
            return None;
        };
        let igneous = env::var("EH_AUTH_IGNEOUS").ok();
        let auth = EhClientAuth::new(&ipb_member_id, &ipb_pass_hash, igneous.as_deref());
        Some(auth)
    }

    /// 输出验证信息为一个键值对向量
    pub fn to_vec(&self) -> Vec<(String, String)> {
        let mut vec = Vec::new();
        vec.push(("ipb_member_id".to_string(), self.ipb_member_id.to_string()));
        vec.push(("ipb_pass_hash".to_string(), self.ipb_pass_hash.to_string()));
        if let Some(igneous) = &self.igneous {
            vec.push(("igneous".to_string(), igneous.to_string()));
        }
        vec
    }
}

impl fmt::Display for EhClientAuth {
    /// 将 EhClientAuth 实例转换为字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ipb_member_id={}&ipb_pass_hash={}",
            self.ipb_member_id, self.ipb_pass_hash
        )?;
        if let Some(igneous) = &self.igneous {
            write!(f, "&igneous={}", igneous)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::client::auth::EhClientAuth;

    #[test]
    fn auth_to_string() {
        let auth = EhClientAuth::new("123456", "123456", Some("abcdef"));
        assert_eq!(
            auth.to_string(),
            "ipb_member_id=123456&ipb_pass_hash=123456&igneous=abcdef"
        );
    }
}
