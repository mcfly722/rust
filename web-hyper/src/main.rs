/*

Golang:
./wrk -t256 -c512 -d30s http://127.0.0.1:8080/health
Running 30s test @ http://127.0.0.1:8080/health
  256 threads and 512 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.44ms    2.79ms  74.04ms   76.82%
    Req/Sec   366.82     55.33     1.43k    71.80%
  2826551 requests in 30.10s, 334.26MB read
Requests/sec:  93899.28
Transfer/sec:     11.10MB


Rust (Hyper)
root@DESKTOP-OT07QOO:/mnt/c/Users/Anonymous/go/src/github.com/wrk# ./wrk -t256 -c512 -d30s http://127.0.0.1:8080/health
Running 30s test @ http://127.0.0.1:8080/health
  256 threads and 512 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.57ms    3.00ms  33.26ms   70.60%
    Req/Sec   185.01     20.35   800.00     84.06%
  1426604 requests in 30.10s, 125.17MB read
Requests/sec:  47393.27
Transfer/sec:      4.16MB

*/
use std::net::SocketAddr;
use std::error::Error;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}/health", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(health_handler)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn health_handler(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => Ok(Response::new(Body::from("GET Hello World!"))),
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}