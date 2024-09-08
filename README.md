![cdlogo](https://carefuldata.com/images/cdlogo.png)

# rocky-bastard

This is a monorepo of tools, services, and configurations for running (Rocky Linux) based systems and a rust monolith in FIPS mode.

More detailed documenting will be added to this file eventually.

## The Rocky Linux server configuration

There is shell script for the configurations. Ansible was originally planned, but there were too many potential complications with FIPS compatibility, so Ansible was removed.

More will be added to these aspects soon!

## The monolith template

This template uses `aws-lc-rs` cryptography via `actix-web` and `rustls`. The only supported architecture target is x86_64 linux at this time, because of upstream limitations.  

## The PKCS12 generation tool genpkcs12

This tool is a non-interactive way to quickly generate ephemeral self signed, encrypted at rest certificate and keypairs.
The real PKCS12 file used in production is likely not self-signed like this, but rather integrated with
the appropriate PKI systems.

# "fms" (file metadata syscheck), for FIM functionality

The CLI tool "fms" is a fast way to collect detailed metadata on files and output a JSON.
The fms program collects DAC information, such as permissions and ownership, and also reports
on inode, size, date, unix metadata (but not selinux data), as well as a SHA3 SHAKE256 checksum.
If there is an issue/risk for certification with the SHA3 code, then it can be safely cut
and the "fms" program can still serve great value. The "byte distribution" and "bit count" metdata
values are very granular metadata in combination with modified time can be strong indications
of tampering.

Additional scripting or programs that wrap around "fms" are typically used to create a full solution.
A script for this purpose is included: `fim.sh` is a wrapper for fms that executes fms against targets
defined in `fimsys.sh`, writing the output to syslog (logger) and json files to the disk in `/opt/fimsyscheck`.
