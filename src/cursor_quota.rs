use crate::response::CursorRes;
use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderName};
use tap::Tap;
use thin_logger::log;

pub struct CursorQuota {
	client: reqwest::Client,
	api_url: String,
	headers: HeaderMap,
}

impl CursorQuota {
	pub fn try_new() -> anyhow::Result<Self> {
		let cookie_value =
			std::env::var("CURSOR_COOKIE").context("CURSOR_COOKIE is not set")?;
		Ok(Self {
			client: reqwest::Client::new(),
			api_url: "https://www.cursor.com/api/usage".to_string(),
			headers: HeaderMap::from_iter(vec![(
				HeaderName::from_static("cookie"),
				cookie_value.parse()?,
			)]),
		})
	}

	pub async fn get_quota(&self) -> anyhow::Result<CursorRes> {
		log::info!("calling url: {}", self.api_url);
		let res = self
			.client
			.get(&self.api_url)
			.headers(self.headers.clone())
			.tap_borrow(|req| log::debug!("req: {:#?}", req))
			.send()
			.await
			.context("Failed to send request to Cursor API")?
			.tap_borrow(|res| log::info!("request status code: {}", res.status()))
			.json::<CursorRes>()
			.await?;
		Ok(res)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_get_quota() {
		dotenvy::dotenv().ok();
		let quota = CursorQuota::try_new().unwrap();
		let res = quota.get_quota().await.unwrap();
		assert_eq!(
			res.premium_reqs.max_request_usage, 500,
			"Premium quota is not 500"
		);
	}
}
