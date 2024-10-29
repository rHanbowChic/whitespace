use crate::bs::Bs;
use crate::incubation::Incubator;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

pub struct Aquifer {
    incubator: Incubator,
    bs: Bs,
    host: String,
}


impl Aquifer {
    pub fn new(url_with_host: &str) -> Aquifer {
        Aquifer {
            incubator: Incubator::new(),
            bs: Bs::new(url_with_host),
            host: url::Url::parse(url_with_host).expect("Aquifer: Err parsing url")
                .host().expect("Aquifer: Err finding host in url").to_string(),
        }
    }
    pub async fn set_text(&mut self, namespace: &str, page: &str, text:&str)
        -> Result<String, Box<dyn std::error::Error>>{
        let mapping = self.incubator.get_mapping(&*self.host, namespace, page);
        let encrypted_bytes = self.incubator.encrypt_with_raw(
            &*self.host,
            namespace,
            page,
            text,
        );
        let encrypted_b64 = BASE64_STANDARD.encode(&encrypted_bytes);
        Ok(self.bs.post(&*mapping, &*encrypted_b64).await?)
    }
    pub async fn get_text(&mut self, namespace: &str, page: &str)
        -> Result<String, Box<dyn std::error::Error>> {
        let mapping = self.incubator.get_mapping(&*self.host, namespace, page);
        let b64 = self.bs.get(&*mapping).await?;
        let text = BASE64_STANDARD.decode(&*b64);
        match text {
            Ok(r) => {
                Ok(self.incubator.decrypt_with_raw(
                    &*self.host,
                    namespace,
                    page,
                    &r
                ))
            }
            Err(_) => {
                Ok("".to_string())
            }
        }


    }

    pub fn get_actual_page(&mut self, namespace: &str, page: &str) -> String {
        let mapping = self.incubator.get_mapping(&*self.host, namespace, page);
        mapping
    }

    pub fn set_text_sync(&mut self, namespace: &str, page: &str, text:&str)
        -> Result<String, Box<dyn std::error::Error>> {
        Ok(tokio::runtime::Runtime::new().unwrap().block_on(self.set_text(namespace, page, text))?)
    }

    pub fn get_text_sync(&mut self, namespace: &str, page: &str)
        -> Result<String, Box<dyn std::error::Error>> {
        Ok(tokio::runtime::Runtime::new().unwrap().block_on(self.get_text(namespace, page))?)
    }
}



