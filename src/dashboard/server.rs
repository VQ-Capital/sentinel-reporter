#[cfg(feature = "dashboard")]
use axum::{
    routing::get,
    Router,
    response::{Html, sse::Event, Sse},
    extract::State,
};
#[cfg(feature = "dashboard")]
use std::convert::Infallible;
#[cfg(feature = "dashboard")]
use std::time::Duration;
#[cfg(feature = "dashboard")]
use tokio::sync::broadcast;
#[cfg(feature = "dashboard")]
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
#[cfg(feature = "dashboard")]
use tower_http::cors::CorsLayer;

#[cfg(feature = "dashboard")]
pub async fn start_server(port: u16, csv_path: String) -> anyhow::Result<()> {
    let (tx, _rx) = broadcast::channel::<String>(10);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/report", get(report_handler))
        .route("/api/stream", get(sse_handler))
        .layer(CorsLayer::permissive())
        .with_state(tx.clone());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    let tx_clone = tx.clone();
    let csv_path_clone = csv_path.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        let mut last_modified = std::time::SystemTime::UNIX_EPOCH;

        loop {
            interval.tick().await;

            if let Ok(metadata) = tokio::fs::metadata(&csv_path_clone).await {
                if let Ok(modified) = metadata.modified() {
                    if modified > last_modified {
                        last_modified = modified;

                        use crate::analyzer::parser::CsvParser;
                        use crate::reporter::markdown::MarkdownReporter;

                        if let Ok(session) = CsvParser::parse_file(&csv_path_clone) {
                            let report = MarkdownReporter::generate(&session);
                            let _ = tx_clone.send(report);
                        }
                    }
                }
            }
        }
    });

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(feature = "dashboard")]
async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../../templates/dashboard.html"))
}

#[cfg(feature = "dashboard")]
async fn report_handler(State(tx): State<broadcast::Sender<String>>) -> Html<String> {
    let mut rx = tx.subscribe();
    match rx.recv().await {
        Ok(report) => Html(report),
        Err(_) => Html("<h1>Henüz rapor yok</h1>".to_string()),
    }
}

#[cfg(feature = "dashboard")]
async fn sse_handler(
    State(tx): State<broadcast::Sender<String>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx);

    let mapped = stream.map(|msg| {
        match msg {
            Ok(data) => Ok(Event::default().data(data).event("report")),
            Err(_) => Ok(Event::default().data("").event("heartbeat")),
        }
    });

    Sse::new(mapped).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive")
    )
}

#[cfg(not(feature = "dashboard"))]
pub async fn start_server(_port: u16, _csv_path: String) -> anyhow::Result<()> {
    Ok(())
}
