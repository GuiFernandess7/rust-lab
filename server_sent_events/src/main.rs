use axum::{
    routing::get,
    response::sse::{Event, KeepAlive, Sse},
    Router,
};
use std::{convert::Infallible, time::Duration};
use futures::Stream;
use tokio_stream::{wrappers::IntervalStream, StreamExt};
use tokio::time::interval;
use chrono::Utc;

mod api;

async fn get_data() -> String {
    //format!("New data at {}", Utc::now().format("%Y-%m-%d %H:%M:%S"))
    return api::call_endpoint("http://api.olhovivo.sptrans.com.br/v2.1/Posicao").await
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let interval = interval(Duration::from_secs(5));
    let stream = IntervalStream::new(interval)
    .then(|_| async {
        let data = get_data().await; // aguarda a Future terminar
        Ok(Event::default().data(data))
    });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(5)))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/events", get(sse_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on http://localhost:3000/events");
    axum::serve(listener, app).await.unwrap();
}
