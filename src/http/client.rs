use crate::api::models::shared::{RequestError, Response};
use crate::api::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, from_value, Value};
use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub struct Client {
    pub host: String,
    client: ::reqwest::Client,
    headers: HeaderMap,
}

impl Client {
    pub fn new(host: &str) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));

        Client {
            host: host.to_owned(),
            client: ::reqwest::Client::new(),
            headers,
        }
    }

    pub async fn get<T: DeserializeOwned + Default>(&mut self, endpoint: &str) -> Result<T> {
        let url = Url::parse(&format!("{}{}", self.host, endpoint)).unwrap();
        self.generic_get(&url).await
    }

    pub async fn get_with_params<T, I, K, V>(&mut self, endpoint: &str, parameters: I) -> Result<T>
    where
        T: DeserializeOwned + Default,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url =
            Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters).unwrap();
        self.generic_get(&url).await
    }

    pub async fn post<T, B>(&self, endpoint: &str, payload: &B) -> Result<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + ?Sized,
    {
        let url = Url::parse(&format!("{}{}", self.host, endpoint)).unwrap();
        self.generic_post(&url, payload).await
    }

    pub async fn post_with_params<T, B, I, K, V>(
        &self,
        endpoint: &str,
        payload: &B,
        parameters: I,
    ) -> Result<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + ?Sized,
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let url =
            Url::parse_with_params(&format!("{}{}", self.host, endpoint), parameters).unwrap();
        self.generic_post(&url, payload).await
    }

    pub async fn generic_get<T: DeserializeOwned + Default>(&self, url: &Url) -> Result<T> {
        let builder = self.client.get(url.as_str());

        self.send(builder).await
    }

    pub async fn generic_post<T, B>(&self, url: &Url, payload: &B) -> Result<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + ?Sized,
    {
        let builder = self.client.post(url.as_str()).json(payload);
        self.send(builder).await
    }

    async fn send<T: DeserializeOwned + Default>(&self, builder: RequestBuilder) -> Result<T> {
        let response = builder
            .headers(self.headers.clone())
            .send()
            .await?
            .text()
            .await?;
        let parsed = from_str::<Value>(&response)?;

        // println!("Rust client http: {:#?}", parsed);

        if parsed.is_object() && parsed.as_object().unwrap().contains_key("statusCode") {
            let request_error = from_value::<RequestError>(parsed)?;
            Err(request_error.into())
        } else {
            match from_value::<Response<T>>(parsed) {
                Ok(response) => Ok(response),
                Err(err) => Err(err.into()),
            }
        }
    }
}
