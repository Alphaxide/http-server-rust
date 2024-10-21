use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(Body::from("Hello, World!")))
        }

        (&Method::POST, "/echo") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_text = String::from_utf8(whole_body.to_vec()).unwrap();

            Ok(Response::new(Body::from(format!("Echo: {}", body_text))))
        }
        _ => {
            let not_found = Response::builder()
                .status(404)
                .body(Body::from("Not Found"))
                .unwrap();
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
