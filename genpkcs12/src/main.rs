use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509, X509NameBuilder};
use openssl::x509::extension::{BasicConstraints, KeyUsage, SubjectKeyIdentifier};
use openssl::pkcs12::Pkcs12;
use openssl::stack::Stack;
use rand::Rng;
use std::fs::File;
use std::io::Write;

const KEY_SIZE: u32 = 4096;
const VALIDITY_DAYS: u32 = 60;

fn generate_key() -> PKey<Private> {
    let rsa = Rsa::generate(KEY_SIZE).unwrap();
    PKey::from_rsa(rsa).unwrap()
}

fn generate_cert(key: &PKey<Private>, ca_key: Option<&PKey<Private>>, ca_cert: Option<&X509>, is_ca: bool) -> X509 {
    let mut x509_builder = X509::builder().unwrap();
    let serial_number = BigNum::from_u32(rand::thread_rng().gen()).unwrap();
    x509_builder.set_serial_number(&serial_number.to_asn1_integer().unwrap()).unwrap();

    let mut subject_name = X509NameBuilder::new().unwrap();
    subject_name.append_entry_by_text("CN", "localhost").unwrap();
    let subject_name = subject_name.build();
    x509_builder.set_subject_name(&subject_name).unwrap();

    if let (Some(_ca_key), Some(ca_cert)) = (ca_key, ca_cert) {
        x509_builder.set_issuer_name(ca_cert.subject_name()).unwrap();
    } else {
        x509_builder.set_issuer_name(&subject_name).unwrap();
    }

    x509_builder.set_pubkey(&key).unwrap();
    let not_before = Asn1Time::days_from_now(0).unwrap();
    let not_after = Asn1Time::days_from_now(VALIDITY_DAYS).unwrap();
    x509_builder.set_not_before(&not_before).unwrap();
    x509_builder.set_not_after(&not_after).unwrap();

    if is_ca {
        x509_builder.append_extension(BasicConstraints::new().critical().ca().build().unwrap()).unwrap();
        x509_builder.append_extension(KeyUsage::new().critical().key_cert_sign().crl_sign().build().unwrap()).unwrap();
    } else {
        x509_builder.append_extension(KeyUsage::new().critical().digital_signature().key_encipherment().build().unwrap()).unwrap();
    }

    x509_builder.append_extension(SubjectKeyIdentifier::new().build(&x509_builder.x509v3_context(None, None)).unwrap()).unwrap();

    if let (Some(ca_key), Some(_)) = (ca_key, ca_cert) {
        x509_builder.sign(ca_key, MessageDigest::sha256()).unwrap();
    } else {
        x509_builder.sign(key, MessageDigest::sha256()).unwrap();
    }

    x509_builder.build()
}

fn main() {
    let root_key = generate_key();
    let root_cert = generate_cert(&root_key, None, None, true);

    let intermediate_key = generate_key();
    let intermediate_cert = generate_cert(&intermediate_key, Some(&root_key), Some(&root_cert), true);

    let ee_key = generate_key();
    let ee_cert = generate_cert(&ee_key, Some(&intermediate_key), Some(&intermediate_cert), false);

    let mut rng = rand::thread_rng();
    let pkcs12_password: String = (0..32).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();

    let mut ca_stack = Stack::new().unwrap();
    ca_stack.push(root_cert).unwrap();
    ca_stack.push(intermediate_cert).unwrap();

    let pkcs12 = Pkcs12::builder()
        .ca(ca_stack)
        .pkey(&ee_key)
        .cert(&ee_cert)
        .build2(&pkcs12_password)
        .unwrap();

    let pkcs12_der = pkcs12.to_der().unwrap();
    let pkcs12_path = "identity.p12";
    let mut file = File::create(pkcs12_path).unwrap();
    file.write_all(&pkcs12_der).unwrap();

    println!("PKCS12 file created: {}", pkcs12_path);
    println!("PKCS12 password: {}", pkcs12_password);
}
