use actix_web::{App, HttpServer};
use rustls::ServerConfig;
use std::env;
use std::fs::File;
use std::io::Read;
use aws_lc_rs;
use openssl::pkcs12::Pkcs12;
use rustls::pki_types::PrivateKeyDer;

mod frontend;

fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    aws_lc_rs::try_fips_mode().expect("Failed to initialize FIPS mode");
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    let pkcs12_path = env::var("PKCSPATH")?;
    let pkcs12_password = env::var("PKCSPASSWORD")?;
    let mut file = File::open(pkcs12_path)?;
    let mut pkcs12_data = vec![];
    file.read_to_end(&mut pkcs12_data)?;
    let pkcs12 = Pkcs12::from_der(&pkcs12_data)?;
    let parsed_pkcs12 = pkcs12.parse2(&pkcs12_password)?;
    let cert_chain = parsed_pkcs12.cert.into_iter()
        .map(|cert| rustls::pki_types::CertificateDer::from(cert.to_der().unwrap()))
        .collect::<Vec<_>>();
    let pkey = parsed_pkcs12.pkey.expect("NO PRIVATE KEY FOUND");
    let key_der = pkey.private_key_to_der()?;

    let key = match pkey.id() {
        openssl::pkey::Id::RSA => PrivateKeyDer::Pkcs1(key_der.into()),
        openssl::pkey::Id::EC => PrivateKeyDer::Sec1(key_der.into()),
        openssl::pkey::Id::ED25519 => PrivateKeyDer::Pkcs8(key_der.into()),
        _ => return Err("Unsupported key type".into()),
    };

    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;
    Ok(config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let tls_config = load_tls_config().expect("Failed to load TLS config");
    HttpServer::new(|| App::new().configure(frontend::init_routes))
        .bind_rustls_0_23("0.0.0.0:443", tls_config)?
        .run()
        .await
}
