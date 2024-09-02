![cdlogo](https://carefuldata.com/images/cdlogo.png)

# rocky-bastard

This project contains tools and templating for building servers and services for FIPS (government) compliance.
This is not always a good thing. For example, we are not able to optimize the Rust for compact OCI images when
complying fully with FIPS. But if we do need to run within a container, we can, and then use slim to tighten down.
The example OCI image builds may be included here, but the focus on the repository is on having a service running
on a Rocky Linux server (created in Vultr cloud) that can quickly reach very strict compliance criteria.

This is a monorepo of sorts, with several rust programs as well as Ansible and scripts for configuring the Rocky Linux server/s.

## The Rocky Linux server configuration

There is Anisble and shell script for the configurations. More will be added to these aspects soon!

## The monolith template

This template uses `aws-lc-rs` cryptography via `actix-web` and `rustls`. The only supported architecture target is x86_64 linux at this time, because of upstream limitations.  

## The PKCS12 generation tool

This tool is a non-interactive way to quickly generate compliant encrypted at rest certificate and keypairs.
The real PKCS12 file used in production is likely not self-signed like this, but rather integrated with
the appropriate PKI systems.
