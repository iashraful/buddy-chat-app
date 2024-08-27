use log::info;

pub async fn index() -> &'static str {
    info!("Health Check Requested.");
    "OK!"
}
