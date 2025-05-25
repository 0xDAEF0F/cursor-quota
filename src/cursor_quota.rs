use crate::{jwt, response::CursorRes};
use anyhow::{Context, Result, ensure};
use colored::Colorize as _;
use reqwest::header::{HeaderMap, HeaderName};
use tap::Tap;
use thin_logger::log;

const URL: &str = "https://www.cursor.com/api/usage";

pub struct CursorQuota {
	client: reqwest::Client,
	api_url: &'static str,
	headers: HeaderMap,
}

impl CursorQuota {
	pub fn try_new() -> Result<Self> {
		let jwt_str = std::env::var("JWT").context("JWT env var is not set")?;
		let jwt = jwt::decode_jwt(&jwt_str)?;
		ensure!(jwt.is_valid(), "JWT is expired");
		log::info!("Setting base URL to {URL}");
		Ok(Self {
			client: reqwest::Client::new(),
			api_url: URL,
			headers: HeaderMap::from_iter(vec![(
				HeaderName::from_static("cookie"),
				jwt.get_cookie(&jwt_str)?,
			)]),
		})
	}

	pub async fn get_quota(&self) -> Result<CursorRes> {
		let instant = tokio::time::Instant::now();
		let res = self
			.client
			.get(self.api_url)
			.headers(self.headers.clone())
			.tap_borrow(|req| log::trace!("req: {req:#?}"))
			.send()
			.await
			.context("Failed to send request to Cursor API")?
			.tap_borrow(|res| {
				log::trace!("response: {res:#?}");
				log::info!("response status code: {}", res.status())
			})
			.json::<CursorRes>()
			.await?;
		log::info!(
			"Request took: {}",
			format!("{}ms", instant.elapsed().as_millis()).purple()
		);
		Ok(res)
	}
}
