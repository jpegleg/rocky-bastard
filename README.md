![cdlogo](https://carefuldata.com/images/cdlogo.png)

# rocky-bastard

This is a monorepo of tools, services, and configurations for running (Rocky Linux) based systems and a rust monolith in FIPS mode.

More detailed documenting will be added to this file eventually.

## The Rocky Linux server configuration

There is Anisble and shell script for the configurations. More will be added to these aspects soon!

## The monolith template

This template uses `aws-lc-rs` cryptography via `actix-web` and `rustls`. The only supported architecture target is x86_64 linux at this time, because of upstream limitations.  

## The PKCS12 generation tool genpkcs12

This tool is a non-interactive way to quickly generate ephemeral self signed, encrypted at rest certificate and keypairs.
The real PKCS12 file used in production is likely not self-signed like this, but rather integrated with
the appropriate PKI systems.
