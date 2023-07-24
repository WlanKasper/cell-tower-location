//! Main binary entry point for swagger_client implementation.

#![allow(missing_docs)]

// Imports required by this file.
// extern crate <name of this crate>;
extern crate swagger_client;
extern crate swagger;
extern crate hyper;
extern crate openssl;
extern crate native_tls;
extern crate tokio_proto;
extern crate tokio_tls;
extern crate clap;

// Imports required by server library.
// extern crate swagger_client;
// extern crate swagger;
extern crate futures;
extern crate chrono;
#[macro_use]
extern crate error_chain;


use openssl::x509::X509_FILETYPE_PEM;
use openssl::ssl::{SslAcceptorBuilder, SslMethod};
use openssl::error::ErrorStack;
use hyper::server::Http;
use tokio_proto::TcpServer;
use clap::{App, Arg};
use swagger::auth::AllowAllAuthenticator;
use swagger::EmptyContext;

mod server_lib;

// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
fn ssl() -> Result<SslAcceptorBuilder, ErrorStack> {
    let mut ssl = SslAcceptorBuilder::mozilla_intermediate_raw(SslMethod::tls())?;

    // Server authentication
    ssl.set_private_key_file("examples/server-key.pem", X509_FILETYPE_PEM)?;
    ssl.set_certificate_chain_file("examples/server-chain.pem")?;
    ssl.check_private_key()?;

    Ok(ssl)
}

/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
fn main() {
    let matches = App::new("server")
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .get_matches();

    let service_fn =
        swagger_client::server::auth::NewService::<_, EmptyContext>::new(
            AllowAllAuthenticator::new(
                server_lib::NewService::new(),
                "cosmo"
            )
        );

    let addr = "127.0.0.1:33999".parse().expect("Failed to parse bind address");
    if matches.is_present("https") {
        let ssl = ssl().expect("Failed to load SSL keys");
        let builder: native_tls::TlsAcceptorBuilder = native_tls::backend::openssl::TlsAcceptorBuilderExt::from_openssl(ssl);
        let tls_acceptor = builder.build().expect("Failed to build TLS acceptor");
        TcpServer::new(tokio_tls::proto::Server::new(Http::new(), tls_acceptor), addr).serve(service_fn);
    } else {
        // Using HTTP
        TcpServer::new(Http::new(), addr).serve(service_fn);
    }
}
