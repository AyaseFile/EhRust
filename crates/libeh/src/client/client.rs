use reqwest::{cookie::Jar, redirect, Client, Proxy, Url};
use serde::de::DeserializeOwned;

use crate::dto::{keyword::Keyword, search_offset::Offset, site::Site};
use crate::url::search::SearchBuilder;

use super::config::EhClientConfig;

#[derive(Clone)]
pub struct EhClient {
    site: Site,
    client: Client,
}

impl EhClient {
    pub fn new(config: EhClientConfig) -> Self {
        let mut builder = Client::builder()
            .redirect(redirect::Policy::limited(20))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0");
        if let Some(proxy) = config.proxy.clone() {
            let proxy = Proxy::all(proxy.to_string());
            if let Ok(proxy) = proxy {
                builder = builder.proxy(proxy);
            }
        }
        if let Some(auth) = config.auth.clone() {
            let jar = Jar::default();
            let auth_vec = auth.to_vec();
            for (key, value) in auth_vec {
                jar.add_cookie_str(&format!("{}={}", key, value), &config.site.into());
            }
            // println!("Cookie: {:?}", jar);
            builder = builder.cookie_store(true).cookie_provider(jar.into());
        }

        let client = builder.build();

        match client {
            Ok(client) => EhClient {
                client,
                site: config.site,
            },
            Err(err) => panic!("Error: {}", err),
        }
    }

    /// 不包含高级选项的搜索
    pub async fn search(
        &self,
        keywords: Vec<Keyword>,
        offset: Option<Offset>,
    ) -> Result<String, String> {
        let mut builder = SearchBuilder::new(self.site).add_keywords(keywords);
        if let Some(offset) = offset {
            builder = builder.offset(offset);
        }
        let url = builder.build()?;
        let text = match self.get_html(url).await {
            Ok(text) => text,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        Ok(text)
    }

    pub async fn get_html(&self, url: Url) -> Result<String, String> {
        let res: Result<reqwest::Response, reqwest::Error> = self.client.get(url).send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        let text = match res.text().await {
            Ok(text) => text,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        Ok(text)
    }

    pub async fn get_json<T>(&self, url: Url) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let res: Result<reqwest::Response, reqwest::Error> = self.client.get(url).send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        let json = match res.json::<T>().await {
            Ok(json) => json,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        Ok(json)
    }

    pub async fn post_json<T, R>(&self, url: Url, body: T) -> Result<R, String>
    where
        T: Into<reqwest::Body>,
        R: DeserializeOwned,
    {
        let res = self.client.post(url).body(body).send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        let json = match res.json::<R>().await {
            Ok(json) => json,
            Err(err) => {
                return Err(format!("Error: {}", err));
            }
        };
        Ok(json)
    }

    /// 发送表单数据并获取响应文本
    pub async fn post_form(
        &self,
        url: Url,
        form_data: Vec<(&str, &str)>,
    ) -> Result<String, String> {
        let res = self.client.post(url).form(&form_data).send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => return Err(format!("Error: {}", err)),
        };
        let text = match res.text().await {
            Ok(text) => text,
            Err(err) => return Err(format!("Error: {}", err)),
        };
        Ok(text)
    }

    /// 获取指定 URL 的二进制内容
    pub async fn get_bytes(&self, url: Url) -> Result<Vec<u8>, String> {
        let res: Result<reqwest::Response, reqwest::Error> = self.client.get(url).send().await;
        let res = match res {
            Ok(res) => res,
            Err(err) => return Err(format!("Error: {}", err)),
        };
        let bytes = match res.bytes().await {
            Ok(bytes) => bytes.to_vec(),
            Err(err) => return Err(format!("Error: {}", err)),
        };
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::{config::EhClientConfig, proxy::EhClientProxy},
        dto::{keyword::Keyword, site::Site},
    };

    use super::EhClient;
    use tokio::{fs::File, io::AsyncWriteExt};

    #[tokio::test]
    async fn test_eh_client() {
        let proxy = if dotenvy::dotenv().is_ok() {
            EhClientProxy::env()
        } else {
            Some(EhClientProxy::new("http", "127.0.0.1", 7897))
        };
        let config = EhClientConfig {
            site: Site::Eh,
            proxy,
            auth: None,
        };
        let client = EhClient::new(config);
        let res = client
            .search(
                vec![
                    Keyword::Artist("simon".into()),
                    Keyword::Language("chinese".into()),
                ],
                None,
            )
            .await;
        match res {
            Ok(text) => {
                // println!("HTML: {}", text);
                let mut cwd = std::env::current_dir().unwrap();
                cwd.push("../../samples/search.html");
                println!("File: {}", cwd.display());
                let file = File::create(cwd).await;
                let mut file = match file {
                    Ok(file) => file,
                    Err(err) => panic!("Error: {}", err),
                };
                let result = file.write(text.as_bytes()).await;
                assert!(result.is_ok());
            }
            Err(err) => panic!("Error: {}", err),
        }
    }
}
