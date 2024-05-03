pub mod middleware {
    use axum::{
        body::Body,
        http::Request,
        middleware::Next,
        response::Response,
    };
    
    pub async fn request_logging_middleware(req: Request<Body>, next: Next) -> Response {
        let instant = std::time::Instant::now();
        let req_info = format!("{:?} {} {}", req.version(), req.method(), req.uri());
        let res = next.run(req).await;
        log::info!("{} {} {}ms", req_info, res.status(), instant.elapsed().as_millis());
        res
    }

    // pub async fn global_error_handler_middleware(err: anyhow::Error) -> Response {
    //     log::error!("Internal server error: {:?}", err);
    //     Response::builder()
    //         .status(500)
    //         .body(Body::empty())
    //         .unwrap()
    // }
}