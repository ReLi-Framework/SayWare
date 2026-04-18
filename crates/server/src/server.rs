// SPDX-FileCopyrightText: 2026 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::{api::routes::Sentence, configuration::Configuration};
use color_eyre::eyre::{Result, WrapErr, eyre};
use poem::{
    Endpoint, EndpointExt, Route,
    listener::{Acceptor, Listener, RustlsCertificate, RustlsConfig, TcpListener},
    middleware::Tracing,
};
use poem_openapi::OpenApiService;
use tokio::signal;

pub struct Tls {
    pub certificate: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub struct Server;

impl Server {
    pub async fn run(configuration: Configuration) -> Result<()> {
        let Configuration {
            port,
            endpoint,
            sentence,
            tls,
        } = configuration;

        let api = OpenApiService::new(
            Sentence::new(sentence.clone()),
            "SayWare Server",
            env!("CARGO_PKG_VERSION"),
        );
        let application = Route::new().nest(format!("/{endpoint}"), api).with(Tracing);

        let listener = TcpListener::bind(format!("0.0.0.0:{port}"));
        let https = tls.is_some();

        match tls {
            Some(Tls {
                certificate,
                private_key,
            }) => {
                let configuration = RustlsConfig::new()
                    .fallback(RustlsCertificate::new().cert(certificate).key(private_key));
                let acceptor = listener
                    .rustls(configuration)
                    .into_acceptor()
                    .await
                    .wrap_err("failed to bind HTTPS listener")?;
                Self::serve(acceptor, application, &endpoint, &sentence, https).await
            }
            None => {
                let acceptor = listener
                    .into_acceptor()
                    .await
                    .wrap_err("failed to bind HTTP listener")?;
                Self::serve(acceptor, application, &endpoint, &sentence, https).await
            }
        }
    }

    async fn serve<A, E>(
        acceptor: A,
        application: E,
        endpoint: &str,
        sentence: &str,
        https: bool,
    ) -> Result<()>
    where
        A: Acceptor + 'static,
        E: Endpoint + 'static,
    {
        Self::announce_ready(&acceptor, endpoint, sentence, https)?;

        poem::Server::new_with_acceptor(acceptor)
            .run_with_graceful_shutdown(application, Self::shutdown_signal(), None)
            .await
            .wrap_err("server stopped with an error")
    }

    async fn shutdown_signal() {
        signal::ctrl_c()
            .await
            .expect("failed to install SIGINT handler");
    }

    fn announce_ready(
        acceptor: &impl Acceptor,
        endpoint: &str,
        sentence: &str,
        https: bool,
    ) -> Result<()> {
        let local_addresses = acceptor.local_addr();
        let bound_address = local_addresses
            .first()
            .and_then(|address| address.as_socket_addr().cloned())
            .ok_or_else(|| eyre!("listener did not report a local socket address"))?;

        let (scheme, https_label) = if https {
            ("https", "enabled")
        } else {
            ("http", "disabled")
        };
        let summary = format!(
            "Address:  {scheme}://{bound_address}\n\
         Endpoint: /{endpoint}\n\
         HTTPS:    {https_label}\n\
         Sentence: {sentence:?}",
        );

        cliclack::note("SayWare server ready", summary)?;

        Ok(())
    }
}
