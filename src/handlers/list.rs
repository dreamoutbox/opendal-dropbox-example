use std::time::Instant;

use anyhow::{Context, Result};
use opendal::{EntryMode, Operator};

/// List files under the specified path, printing a summary table of items found.
pub(crate) async fn handle_list(op: &Operator, path: &str, recursive: bool) -> Result<()> {
    let start = Instant::now();
    let display_path = if path.is_empty() { "/" } else { path };
    println!(
        "Listing Dropbox entries under '{}' (recursive: {})...",
        display_path, recursive
    );

    let entries = op
        .list_with(path)
        .recursive(recursive)
        .await
        .with_context(|| format!("Failed to list entries under '{path}'"))?;

    let elapsed = start.elapsed();
    println!(
        "[OK] Found {} entry/entries in {:.2?}:",
        entries.len(),
        elapsed
    );

    if entries.is_empty() {
        println!("  (No items found)");
        return Ok(());
    }

    for (idx, entry) in entries.iter().enumerate() {
        let meta = entry.metadata();
        let mode_str = match meta.mode() {
            EntryMode::FILE => "FILE",
            EntryMode::DIR => "DIR ",
            EntryMode::Unknown => "UNK ",
        };

        let size_str = if meta.mode().is_file() {
            format!("{} bytes", meta.content_length())
        } else {
            "-".to_string()
        };

        println!(
            "  [{:02}] {:<4} | {:<12} | {}",
            idx + 1,
            mode_str,
            size_str,
            entry.path()
        );
    }

    Ok(())
}
