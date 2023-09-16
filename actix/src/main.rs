/*

Web wrk performance tests:

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


Rust:
./wrk -t256 -c512 -d30s http://127.0.0.1:8080/health
Running 30s test @ http://127.0.0.1:8080/health
  256 threads and 512 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.51ms    2.59ms  35.24ms   76.40%
    Req/Sec   355.55     50.87     1.26k    69.92%
  2740714 requests in 30.10s, 324.10MB read
Requests/sec:  91044.76
Transfer/sec:     10.77MB

*/
use actix_web::{web, Responder, middleware, App, HttpServer};

async fn health_check() -> impl Responder {
    "Welcome!"
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .configure(routes)
    });
    serv.bind("127.0.0.1:8080")?
        .run()
        .await
}