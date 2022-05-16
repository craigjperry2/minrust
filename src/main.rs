use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Config {
    #[clap(short, long, default_value_t = 8080)]
    port: i16,
}

#[tokio::main]
async fn main() {
    let config = parse_args();
    println!("Starting server on port {}", config.port);

    let router = make_router();

    axum::Server::bind(
        &format!("0.0.0.0:{}", config.port).parse().unwrap()
    ).serve(router.into_make_service())
    .await
    .unwrap();
}

fn parse_args() -> Config {
    Config::parse()
}

fn make_router() -> axum::Router {
    axum::Router::new().route("/", axum::routing::get(|| async { "Hello, World!" }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn config_has_a_default_port() {
        let config = crate::parse_args();
        assert_eq!(config.port, 8080);
    }
}
