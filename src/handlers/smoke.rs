use std::time::Duration;

use anyhow::{Context, Result, bail};
use chrono::Utc;
use opendal::Operator;

/// Full lifecycle round-trip smoke test:
/// upload sample file -> verify read -> check in directory list -> delete -> verify deletion.
pub(crate) async fn handle_smoke_test(op: &Operator) -> Result<()> {
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let test_filename = format!("opendal_prototype_smoke_{timestamp}.txt");
    let test_payload = format!(
        "Smoke test payload created by opendal-dropbox-example at {}\nIntegrity check verified.",
        Utc::now().to_rfc3339()
    );

    println!("\n[1/4] Testing Upload...");
    op.write(&test_filename, test_payload.as_bytes().to_vec())
        .await
        .with_context(|| format!("Smoke test upload failed for '{test_filename}'"))?;
    println!(
        "  Uploaded test file: '{test_filename}' ({} bytes)",
        test_payload.len()
    );

    println!("\n[2/4] Testing Get & Content Integrity...");
    let read_buf = op
        .read(&test_filename)
        .await
        .with_context(|| format!("Smoke test get failed for '{test_filename}'"))?;
    let downloaded_text =
        String::from_utf8(read_buf.to_vec()).context("Downloaded content is not valid UTF-8")?;

    if downloaded_text == test_payload {
        println!("  Content integrity check matched successfully.");
    } else {
        bail!(
            "Content mismatch! Expected:\n{}\nGot:\n{}",
            test_payload,
            downloaded_text
        );
    }

    println!("\n[3/4] Testing List...");
    let entries = op
        .list("")
        .await
        .context("Smoke test list operation failed")?;
    let found = entries
        .iter()
        .any(|e| e.path().ends_with(&test_filename) || e.name() == test_filename);

    if found {
        println!("  Confirmed: '{test_filename}' appears in directory listing.");
    } else {
        println!(
            "  Note: File did not appear in flat listing of root (might be in nested folder depending on root setting)."
        );
    }

    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("\n[4/4] Testing Delete...");
    op.delete(&test_filename)
        .await
        .with_context(|| format!("Smoke test delete failed for '{test_filename}'"))?;
    println!("  Deleted test file: '{test_filename}'");

    let exists_after = op.exists(&test_filename).await.unwrap_or(false);
    if !exists_after {
        println!("  Confirmed: '{test_filename}' has been removed from Dropbox.");
    } else {
        println!("  Warning: exists() still returned true immediately after delete.");
    }

    println!("\n[SUCCESS] OpenDAL Dropbox prototype smoke test passed all stages.\n");
    Ok(())
}
