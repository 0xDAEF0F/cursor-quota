use anyhow::{Context as _, Result};
use base64::prelude::*;
use chrono::Utc;
use colored::Colorize;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use thin_logger::log;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct JwtPayload {
	sub: String,
	time: String,
	randomness: String,
	exp: i64,
	iss: String,
	scope: String,
	aud: String,
	r#type: String,
}

impl JwtPayload {
	pub fn is_valid(&self) -> bool {
		let exp_date = chrono::DateTime::from_timestamp(self.exp, 0)
			.expect("timestamp must be valid")
			.format("%d/%m/%Y")
			.to_string()
			.blue();
		let days_from_now = if self.exp > Utc::now().timestamp() {
			format!("(~{} days)", ((self.exp - Utc::now().timestamp()) / 86_400))
				.to_string()
				.yellow()
		} else {
			"[EXPIRED]".to_string().red()
		};
		log::info!("jwt expires: {exp_date} {days_from_now}");
		self.exp > Utc::now().timestamp()
	}

	pub fn get_cookie(&self, jwt: &str) -> Result<HeaderValue> {
		let user_id = self.get_user_id().context("no user id in jwt")?;
		let cookie = format!("WorkosCursorSessionToken={user_id}%3A%3A{jwt}");
		log::debug!("{}: {cookie}", "the cookie is".blue());
		Ok(HeaderValue::from_str(&cookie)?)
	}

	fn get_user_id(&self) -> Option<&str> { self.sub.split('|').nth(1) }
}

pub fn decode_jwt(jwt: &str) -> Result<JwtPayload> {
	let raw_payload = jwt.split('.').nth(1).context("No payload in JWT")?;
	let bytes = BASE64_URL_SAFE_NO_PAD.decode(raw_payload)?;
	let payload = serde_json::from_slice(&bytes)?;
	Ok(payload)
}
