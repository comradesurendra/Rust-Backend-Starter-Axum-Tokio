use std::sync::Arc;
use axum::{http::{Request, Response}, routing::get, Router};
use axum::extract::MatchedPath;
use tower_http::trace::TraceLayer;
use tracing::{info_span, Span};
use std::time::Duration;

use crate::app_state::AppState;

pub mod health_check;
pub mod v1;

pub fn router(state: Arc<AppState>) -> Router {
    let trace_layer = TraceLayer::new_for_http()
        // Create a span with rich context about the request
        .make_span_with(|request: &Request<_>| {
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(|p| p.as_str())
                .unwrap_or("unknown");

            let query = request.uri().query().unwrap_or("");

            info_span!(
                "http_request",
                method = %request.method(),
                path = %request.uri().path(),
                route = %matched_path,
                query = %query,
            )
        })
        // Log the response with status and latency
        .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            // Only log successful responses; errors are logged in on_failure
            if response.status().is_success() {
                tracing::info!(
                    status = %response.status(),
                    latency_ms = latency.as_millis() as u64,
                    "response_sent"
                );
            }
        })
        // Log classified failures (e.g., 5xx or service errors)
        .on_failure(|failure_class, latency: Duration, _span: &Span| {
            tracing::error!(
                error = %failure_class,
                latency_ms = latency.as_millis() as u64,
                "request_failed"
            );
        });

    Router::new()
        .route("/health", get(health_check::health))
        .nest("/api/v1", v1::router())
        .layer(trace_layer)
        .with_state(state)
}


