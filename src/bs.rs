use reqwest::header;

pub struct Bs {
    client: reqwest::Client,
    url_with_host: String,
}

impl Bs {
    pub fn new(host: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("curl/8"));

        Bs {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .http2_adaptive_window(true)
                .use_rustls_tls()
                .build().expect("Error building headers"),
            url_with_host: {
                let mut s = host.to_string();
                if !s.ends_with("/") { s += "/"; }
                s
            }
        }
    }

    pub async fn get(&self, page: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.client.get({
            self.url_with_host.clone() + page
        })
            .send().await?.text().await?
        )
    }

    pub async fn post(&self, page: &str, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let text = text.to_string();
        let form = reqwest::multipart::Form::new().text("t", text);
        Ok(self.client.post(self.url_with_host.clone() + page)
            .multipart(form)
            .headers({
                let mut headers = header::HeaderMap::new();
                headers.insert("User-Agent", header::HeaderValue::from_static("whitespace/1"));
                headers.insert("Referer", header::HeaderValue::from_str((self.url_with_host.clone() + page).as_str()).expect("Error parsing header value"));
                headers
            })
            .send().await?.status().to_string())

    }

    pub fn get_sync(&self, page: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(tokio::runtime::Runtime::new().unwrap().block_on(self.get(page))?)
    }

    pub fn post_sync(&self, page: &str, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(tokio::runtime::Runtime::new().unwrap().block_on(self.post(page, text))?)
    }
}
