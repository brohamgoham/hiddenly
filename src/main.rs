
use axum::{http::StatusCode, response::Html, routing::{post, get}, Json, Router};
use types::User;
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};

mod types;



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_|
        {
            "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()

        }
        )).with(tracing_subscriber::fmt::layer()).init();

    let app = Router::new()
        .route("/v1/users", post(create_user)).layer(TraceLayer::new_for_http())
        .route("/", get(heartbeat)).layer(TraceLayer::new_for_http());

    let server = tokio::net::TcpListener::bind("0.0.0.0:6969").await.unwrap();

    tracing::debug!("listening on {}", server.local_addr().unwrap());

    axum::serve(server, app).await.unwrap()
}


async fn create_user(Json(paylod): Json<User>) -> (StatusCode, Json<User>) {
    let user = User::new(&paylod.fname, &paylod.lname, &paylod.email);

    (StatusCode::from_u16(200).unwrap(), Json(user))
}

async fn heartbeat() -> Html<&'static str> {
    Html("<h3>Keep Alive</h1>")
}