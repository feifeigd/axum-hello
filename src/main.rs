

use axum::{
    extract::{
        Path,
        Query, 
        // State,
    }, 
    response, routing::get, serve, Router
};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use dotenv::dotenv;

use serde_json::{
    json,
    Value
};

use std::{
    collections::HashMap, 
    env, 
    net::SocketAddrV4, str::FromStr, sync::Arc
};

use tokio::net::TcpListener;

struct AppState {
    // Add shared state fields here
}

fn init_logging(){
    tracing_subscriber::fmt().init();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_logging();

    // 数据库连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    let shared_state = Arc::new(AppState {
        // Initialize shared state fields here
    });

    let app = Router::new()
        .with_state(shared_state)
        .route("/", get(|| async { "Hello, world!" }))
        .route("/json", get(json_handler))
        .route("/plain", get(plain_text_handler))
        .route("/query", get(query_handler))
        .route("/users/{id}", get(path_handler));
    let addr = SocketAddrV4::from_str("127.0.0.1:3000").unwrap();
    tracing::info!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

async fn path_handler(Path(user_id): Path<u32>) -> String {
    format!("Hello, user {}!", user_id)
}

async fn query_handler(Query(params): Query<HashMap<String, String>>) -> String {
    format!("Hello, {}!", params.get("name").unwrap_or(&"stranger".into()))
}

async fn plain_text_handler() -> &'static str {
    "Hello, plain text!"
}

async fn json_handler() -> response::Json<Value> {
    response::Json(json!({
        "message": "Hello, JSON!"
    }))
}

// async fn shared_state_handler(State(state): State<Arc<AppState>>) -> String {
//     format!("Hello, shared state!")
// }
