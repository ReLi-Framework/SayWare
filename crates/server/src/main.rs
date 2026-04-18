// SPDX-FileCopyrightText: 2026 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod api;
mod configuration;
mod server;

use crate::configuration::Configuration;
use color_eyre::eyre::{Report, Result};
use std::io;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // Swallow SIGINT so crossterm can deliver Ctrl+C to cliclack prompts.
    ctrlc::set_handler(|| {}).ok();

    cliclack::intro("SayWare Server")?;

    let configuration = match Configuration::prompt() {
        Ok(configuration) => configuration,
        Err(error) if is_interrupted(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    let Configuration {
        port,
        endpoint,
        sentence,
        tls,
    } = configuration;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    Ok(())
}

fn is_interrupted(error: &Report) -> bool {
    error
        .chain()
        .filter_map(|source| source.downcast_ref::<io::Error>())
        .any(|io_error| io_error.kind() == io::ErrorKind::Interrupted)
}
