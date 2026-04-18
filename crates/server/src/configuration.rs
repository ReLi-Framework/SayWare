// SPDX-FileCopyrightText: 2026 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::server::Tls;
use color_eyre::eyre::{Result, WrapErr};
use std::{fs, path::Path, result};

pub struct Configuration {
    pub port: u16,
    pub endpoint: String,
    pub sentence: String,
    pub tls: Option<Tls>,
}

impl Configuration {
    pub fn prompt() -> Result<Self> {
        let port = Self::prompt_port()?;
        let endpoint = Self::prompt_endpoint()?;
        let sentence = Self::prompt_sentence()?;

        let enable_https = cliclack::confirm("Enable HTTPS?")
            .initial_value(false)
            .interact()
            .wrap_err("failed to read HTTPS mode")?;

        let tls = enable_https.then(Self::prompt_tls).transpose()?;

        Ok(Self {
            port,
            endpoint,
            sentence,
            tls,
        })
    }

    fn prompt_port() -> Result<u16> {
        let automatic = cliclack::confirm("Let the operating system pick the port automatically?")
            .initial_value(true)
            .interact()
            .wrap_err("failed to read port mode")?;

        if automatic {
            return Ok(0);
        }

        cliclack::input("Listening port:")
            .validate(|value: &String| match value.parse::<u16>() {
                Ok(0) => Err("port must be greater than 0 (use automatic mode for that)"),
                Ok(_) => Ok(()),
                Err(_) => Err("port must be a number between 1 and 65535"),
            })
            .interact::<u16>()
            .wrap_err("failed to read port")
    }

    fn prompt_endpoint() -> Result<String> {
        cliclack::input("Sentence endpoint:")
            .placeholder("sentence")
            .validate(|value: &String| {
                if value.is_empty() {
                    return Err("endpoint must not be empty");
                }

                if value.contains(char::is_whitespace) {
                    return Err("endpoint must not contain whitespace");
                }

                if value.contains(['/', '?', '#', ':']) {
                    return Err("endpoint must be a single URL segment");
                }

                Ok(())
            })
            .interact()
            .wrap_err("failed to read endpoint")
    }

    fn prompt_sentence() -> Result<String> {
        cliclack::input("Sentence to serve:")
            .validate(|value: &String| {
                if value.is_empty() {
                    Err("sentence must not be empty")
                } else {
                    Ok(())
                }
            })
            .interact()
            .wrap_err("failed to read sentence")
    }

    fn prompt_tls() -> Result<Tls> {
        let certificate_path: String = cliclack::input("Path to the PEM certificate chain:")
            .validate(Self::ensure_file_exists)
            .interact()
            .wrap_err("failed to read certificate path")?;

        let private_key_path: String = cliclack::input("Path to the PEM private key:")
            .validate(Self::ensure_file_exists)
            .interact()
            .wrap_err("failed to read private key path")?;

        let certificate = fs::read(&certificate_path)
            .wrap_err_with(|| format!("failed to read certificate at {certificate_path}"))?;
        let private_key = fs::read(&private_key_path)
            .wrap_err_with(|| format!("failed to read private key at {private_key_path}"))?;

        Ok(Tls {
            certificate,
            private_key,
        })
    }

    fn ensure_file_exists(value: &String) -> result::Result<(), &'static str> {
        if Path::new(value).is_file() {
            Ok(())
        } else {
            Err("path does not point to an existing file")
        }
    }
}
