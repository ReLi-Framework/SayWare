// SPDX-FileCopyrightText: 2026 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use poem_openapi::{OpenApi, payload::PlainText};

pub struct Sentence {
    sentence: String,
}

#[OpenApi]
impl Sentence {
    pub fn new(sentence: String) -> Self {
        Self { sentence }
    }

    /// Return the configured sentence as plain text.
    #[oai(path = "/", method = "get")]
    async fn sentence(&self) -> PlainText<String> {
        PlainText(self.sentence.clone())
    }
}
