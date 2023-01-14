use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Test from Salvo"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:5000"))
        .serve(router)
        .await;
}
