use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use chrono::Utc;
use opendal::Operator;

/// Upload raw bytes or a local file to the remote Dropbox path.
pub(crate) async fn handle_upload(
    op: &Operator,
    remote_path: &str,
    inline_content: Option<&str>,
    local_file: Option<&Path>,
) -> Result<()> {
    let payload = match (local_file, inline_content) {
        (Some(src_path), _) => {
            println!("Reading local file for upload: {}", src_path.display());
            tokio::fs::read(src_path)
                .await
                .with_context(|| format!("Failed to read file {}", src_path.display()))?
        }
        (None, Some(text)) => text.as_bytes().to_vec(),
        (None, None) => {
            let default_text = format!(
                "OpenDAL Dropbox prototype probe written at {}",
                Utc::now().to_rfc3339()
            );
            println!("No content or file supplied. Using sample payload: \"{default_text}\"");
            default_text.into_bytes()
        }
    };

    let payload_len = payload.len();
    let start = Instant::now();
    println!(
        "Uploading to Dropbox path '{}' ({} bytes)...",
        remote_path, payload_len
    );

    op.write(remote_path, payload)
        .await
        .with_context(|| format!("Failed to write to '{remote_path}'"))?;

    let elapsed = start.elapsed();
    println!(
        "[OK] Upload complete: '{}' ({} bytes) in {:.2?}",
        remote_path, payload_len, elapsed
    );

    Ok(())
}
