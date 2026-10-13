use std::time::Instant;

use anyhow::{Context, Result};
use opendal::Operator;

/// Delete a file or path in Dropbox and confirm removal.
pub(crate) async fn handle_delete(op: &Operator, remote_path: &str) -> Result<()> {
    let start = Instant::now();
    println!("Deleting Dropbox path '{}'...", remote_path);

    op.delete(remote_path)
        .await
        .with_context(|| format!("Failed to delete '{remote_path}'"))?;

    let elapsed = start.elapsed();
    println!("[OK] Deleted '{}' in {:.2?}", remote_path, elapsed);

    // Verify deletion status
    let still_exists = op.exists(remote_path).await.unwrap_or(false);

    if still_exists {
        println!(
            "Warning: OpenDAL check indicates path '{}' may still exist.",
            remote_path
        );
    } else {
        println!("Verified: '{}' no longer exists.", remote_path);
    }

    Ok(())
}
