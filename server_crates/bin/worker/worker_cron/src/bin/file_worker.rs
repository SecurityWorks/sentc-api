use std::time::Duration;

use server_api::sentc_file_worker;

const INTERVAL_SEC: u64 = 60 * 60;

#[tokio::main]
async fn main()
{
	server_api::start().await;

	let mut interval = tokio::time::interval(Duration::from_secs(INTERVAL_SEC));

	loop {
		interval.tick().await;

		println!("file worker started");

		tokio::spawn(sentc_file_worker::start());
	}
}
