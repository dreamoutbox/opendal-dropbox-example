use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use opendal::Operator;

/// Retrieve a remote Dropbox file, displaying preview or saving to a local file.
pub(crate) async fn handle_get(
    op: &Operator,
    remote_path: &str,
    out_path: Option<&Path>,
) -> Result<()> {
    let start = Instant::now();
    println!("Fetching from Dropbox path '{}'...", remote_path);

    let buffer = op
        .read(remote_path)
        .await
        .with_context(|| format!("Failed to read from '{remote_path}'"))?;

    let bytes = buffer.to_vec();
    let elapsed = start.elapsed();
    println!(
        "[OK] Fetched {} bytes from '{}' in {:.2?}",
        bytes.len(),
        remote_path,
        elapsed
    );

    if let Some(dest) = out_path {
        tokio::fs::write(dest, &bytes)
            .await
            .with_context(|| format!("Failed to save output to {}", dest.display()))?;
        println!("Saved downloaded file to {}", dest.display());
    } else {
        match std::str::from_utf8(&bytes) {
            Ok(valid_text) => {
                println!("--- Content Preview ---");
                println!("{valid_text}");
                println!("--- End of Content ---");
            }
            Err(_) => {
                println!(
                    "File appears to be binary ({} bytes). Omitted text dump; use --out <FILE> to save.",
                    bytes.len()
                );
            }
        }
    }

    Ok(())
}
