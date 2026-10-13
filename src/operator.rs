use anyhow::{Context, Result};
use opendal::Operator;
use opendal::services::Dropbox;

use crate::Cli;

/// Initialize the OpenDAL Operator using either a direct access token or an OAuth refresh token.
pub(crate) fn build_operator(cli: &Cli) -> Result<Operator> {
    // Explicitly install the default HTTP transport required when OpenDAL auto-registration is off
    opendal::install_default();

    let mut builder = Dropbox::default();
    builder = builder.root(&cli.root);

    if let Some(token) = &cli.token {
        builder = builder.access_token(token);
    } else if let (Some(refresh), Some(id), Some(secret)) =
        (&cli.refresh_token, &cli.client_id, &cli.client_secret)
    {
        builder = builder
            .refresh_token(refresh)
            .client_id(id)
            .client_secret(secret);
    } else {
        anyhow::bail!(
            "Missing credentials: provide an access token or (refresh_token, client_id, client_secret)"
        );
    }

    let op = Operator::new(builder).context("Failed to build OpenDAL Operator for Dropbox")?;

    Ok(op)
}
