/// E-Hentai/ExHentai 用户身份验证信息
#[derive(Debug, Clone)]
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
}

impl ToString for EhClientAuth {
    /// 将 EhClientAuth 实例转换为字符串
    fn to_string(&self) -> String {
        let mut s = format!("ipb_member_id={}&ipb_pass_hash={}", self.ipb_member_id, self.ipb_pass_hash);
        if let Some(igneous) = &self.igneous {
            s += &format!("&igneous={}", igneous);
        }
        s
    }
}