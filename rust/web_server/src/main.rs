use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;

async fn index(_req: HttpRequest) -> &'static str {
    "Hello Dev-ES Community!!"
}

async fn sieve(req: HttpRequest) -> HttpResponse {
    match req.match_info().get("number").map(|n| n.parse().ok()) {
        Some(Some(number)) => HttpResponse::Ok().json(_sieve(number)),
        _ => HttpResponse::BadRequest().finish(),
    }
}

fn _sieve(number: u64) -> Vec<u64> {
    let upper_bound = number + 1;
    let mut is_prime = vec![true; upper_bound as usize];
    for i in 2..upper_bound / 2 {
        if is_prime[i as usize] {
            let mut j = i * 2;
            while j < upper_bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter(|val| val.1)
        .map(|val| val.0 as u64)
        .collect()
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
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive(),
            )
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/").to(index))
            .service(web::resource("/sieve/{number}").to(sieve))
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
