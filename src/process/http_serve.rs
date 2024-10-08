use std::{net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::{info, warn};
#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from_str(&format!("127.0.0.1:{}", port))?;
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate();

    let router = Router::new()
        // .route("/*path", get(file_handler))
        .nest_service("/", dir_service)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
    // info!("Reading file {:?}", p);

    // format!("{:?}, {}", state, path)
}
