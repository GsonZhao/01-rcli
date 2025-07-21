use std::path::PathBuf;

use anyhow::Result;
use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;
use tracing::info;

pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    info!("启动HTTP文件服务器，目录: {:?}, 端口: {}", path, port);
    let serve_dir = ServeDir::new(path).append_index_html_on_directories(true); // 自动查找index.html

    let app = Router::new().nest_service("/", get_service(serve_dir));

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("HTTP服务器已启动，访问 http://localhost:{}", port);
    axum::serve(listener, app).await?;
    Ok(())
}
