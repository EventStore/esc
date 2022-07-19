use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;

use super::authorization::Authorization;
use crate::requests::RequestSender;

#[derive(Clone)]
pub struct Client {
    pub authorization: Arc<dyn Authorization + Send + Sync>,
    pub base_url: String,
    pub sender: RequestSender,
}

impl Client {
    pub async fn send_request<B: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        relative_url: String,
        body: Option<&B>,
        use_return_value: Option<R>,
    ) -> crate::Result<R> {
        let url = if relative_url.starts_with('/') {
            format!("{}{}", self.base_url, relative_url)
        } else {
            format!("{}/{}", self.base_url, relative_url)
        };

        self.sender
            .send_request(
                &self.authorization.authorization_header(),
                method,
                url,
                body,
                use_return_value,
            )
            .await
    }
}
