#[cfg(feature = "dashboard")]
pub mod server;

#[cfg(not(feature = "dashboard"))]
pub mod server {
    pub async fn start_server(_port: u16, _csv_path: String) -> anyhow::Result<()> {
        Ok(())
    }
}
