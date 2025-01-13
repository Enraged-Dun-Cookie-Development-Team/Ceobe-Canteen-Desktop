use reqwest::{Client, Method};

use crate::request_models::{fetch_url, ReleaseVersion, Response};

pub struct VersionManager {
    client: Client,
}

impl VersionManager {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl VersionManager {
    pub async fn fetch_version(
        &self,
        version: Option<semver::Version>,
    ) -> Result<ReleaseVersion, ()> {
        let url = fetch_url(version);
        let request = self
            .client
            .request(Method::GET, url)
            .build()
            .expect("Generate Request Failure");

        let resp = self
            .client
            .execute(request)
            .await
            .expect("Failure to send Reqesut");

        let payload = resp
            .json::<Response<ReleaseVersion>>()
            .await
            .expect("Deserialze to json failure");

        let data = payload.data;

        Ok(data)
    }
}
