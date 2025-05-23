mod cursor_quota;
mod response;

use anyhow::Result;
use cursor_quota::CursorQuota;

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok(); // load .env file (with overrides)
	thin_logger::build(None).init(); // init logging

	let quota = CursorQuota::try_new().unwrap();
	let res = quota.get_quota().await.unwrap();

	println!("{}", serde_json::to_string_pretty(&res)?);

	Ok(())
}
