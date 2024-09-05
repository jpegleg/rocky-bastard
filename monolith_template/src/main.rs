use actix_web::{web, App, HttpServer, HttpResponse};
use rustls::{ServerConfig, Certificate, PrivateKey};
use std::env;
use std::fs::File;
use std::io::Read;
use aws_lc_rs;
use openssl::pkcs12::Pkcs12;

mod frontend;

fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    aws_lc_rs::try_fips_mode().expect("Failed to initialize FIPS mode");

    let pkcs12_path = env::var("PKCSPATH")?;
    let pkcs12_password = env::var("PKCSPASSWORD")?;
    let mut file = File::open(pkcs12_path)?;
    let mut pkcs12_data = vec![];
    file.read_to_end(&mut pkcs12_data)?;
    let pkcs12 = Pkcs12::from_der(&pkcs12_data)?;
    let parsed_pkcs12 = pkcs12.parse2(&pkcs12_password)?;
    let cert_chain = parsed_pkcs12.cert.into_iter()
        .map(|cert| Certificate(cert.to_der().unwrap()))
        .collect::<Vec<_>>();
    let key = PrivateKey(parsed_pkcs12.pkey.expect("NO PRIVATE KEY FOUND").private_key_to_der().unwrap());

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;

    Ok(config)
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let tls_config = load_tls_config().expect("Failed to load TLS config");

    HttpServer::new(|| App::new().configure(frontend::init_routes))
        .bind_rustls("0.0.0.0:443", tls_config)?
        .run()
        .await
}
