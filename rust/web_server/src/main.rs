use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello Dev-ES Community!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server_addr = {
        let domain = std::env::var("SERVER_DOMAIN").unwrap_or(String::from("localhost"));
        let port = std::env::var("SERVER_PORT").unwrap_or(String::from("3001"));
        (
            domain,
            port.parse()
                .expect("SERVER_PORT precisa ser um valor entre 0-65535"),
        )
    };

    let server = HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/").to(index))
    })
    .bind(&server_addr)?;

    println!("Servidor iniciado em '{}:{}'", server_addr.0, server_addr.1);
    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello Dev-ES Community!!"##);

        Ok(())
    }
}
