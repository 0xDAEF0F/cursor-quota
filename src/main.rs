mod cursor_quota;
mod jwt;
mod response;

use anyhow::Result;
use cursor_quota::CursorQuota;

#[tokio::main]
async fn main() -> Result<()> {
	dotenvy::dotenv().ok(); // load .env file
	thin_logger::build(None).init(); // init logging

	let quota = CursorQuota::try_new()?;
	let res = quota.get_quota().await?;

	println!("{res}");

	Ok(())
}
