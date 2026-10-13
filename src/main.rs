use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

mod handlers;

use handlers::{handle_delete, handle_get, handle_list, handle_smoke_test, handle_upload};

mod operator;
use operator::build_operator;

/// Throwaway prototype CLI demonstrating OpenDAL Dropbox operations:
/// upload, get, list, and delete.
#[derive(Parser, Debug)]
#[command(
    name = "opendal-dropbox-example",
    about = "OpenDAL Dropbox prototype spike: upload, get, list, delete",
    version = "0.1.0"
)]
struct Cli {
    /// Dropbox access token / developer key.
    /// Can also be passed via DROPBOX_TOKEN, DROPBOX_ACCESS_TOKEN, or DROPBOX_KEY.
    #[arg(
        short = 'k',
        long = "token",
        alias = "key",
        env = "DROPBOX_TOKEN",
        hide_env_values = true
    )]
    token: Option<String>,

    /// Dropbox OAuth2 refresh token (alternative for long-lived access).
    #[arg(long, env = "DROPBOX_REFRESH_TOKEN", hide_env_values = true)]
    refresh_token: Option<String>,

    /// Dropbox OAuth2 client ID (used with refresh_token).
    #[arg(long, env = "DROPBOX_CLIENT_ID", hide_env_values = true)]
    client_id: Option<String>,

    /// Dropbox OAuth2 client secret (used with refresh_token).
    #[arg(long, env = "DROPBOX_CLIENT_SECRET", hide_env_values = true)]
    client_secret: Option<String>,

    /// Root directory in Dropbox. Defaults to root "/".
    #[arg(
        short,
        long,
        env = "DROPBOX_ROOT",
        default_value = "/",
        hide_env_values = true
    )]
    root: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Upload a file or inline text to Dropbox.
    Upload {
        /// Remote destination path in Dropbox (e.g. test.txt or backups/file.tar).
        path: String,

        /// Text content to upload (if --file is not specified).
        #[arg(short, long)]
        content: Option<String>,

        /// Path to a local file to upload.
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Download and display or save a file from Dropbox.
    Get {
        /// Remote path in Dropbox to download.
        path: String,

        /// Optional local file path to save the downloaded content.
        #[arg(short, long)]
        out: Option<PathBuf>,
    },

    /// List entries in Dropbox under the specified path.
    List {
        /// Path prefix or folder to list (defaults to empty string / root).
        #[arg(default_value = "")]
        path: String,

        /// Recursively list nested folders and files.
        #[arg(short, long)]
        recursive: bool,
    },

    /// Delete a file or folder in Dropbox.
    Delete {
        /// Remote path in Dropbox to delete.
        path: String,
    },

    /// Run an end-to-end smoke test (upload -> get -> list -> delete).
    Smoke,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
        )
        .init();

    // Load .env from current directory or ancestors; fallback to opendal-dropbox-example/.env
    if dotenvy::dotenv().is_err() {
        let package_env = std::path::Path::new("opendal-dropbox-example").join(".env");
        if package_env.exists() {
            // Safe to ignore error if the file exists but cannot be parsed as dotenv
            let _ = dotenvy::from_path(&package_env);
        }
    }

    let mut cli = Cli::parse();

    // Check fallback environment variables if token wasn't captured yet
    if cli.token.is_none() {
        if let Ok(key) = std::env::var("DROPBOX_ACCESS_TOKEN") {
            cli.token = Some(key);
        } else if let Ok(key) = std::env::var("DROPBOX_KEY") {
            cli.token = Some(key);
        }
    }

    // Prompt user on stdin if no credentials were provided
    let has_refresh_flow =
        cli.refresh_token.is_some() && cli.client_id.is_some() && cli.client_secret.is_some();

    if cli.token.is_none() && !has_refresh_flow {
        println!("No Dropbox key or access token found in arguments or environment.");
        print!("Please enter your Dropbox Access Token / Key: ");
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("Failed to read token from stdin")?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            bail!("A non-empty Dropbox token is required to proceed.");
        }
        cli.token = Some(trimmed.to_string());
    }

    let op = build_operator(&cli)?;

    match cli.command {
        Some(Commands::Upload {
            path,
            content,
            file,
        }) => handle_upload(&op, &path, content.as_deref(), file.as_deref()).await?,
        Some(Commands::Get { path, out }) => {
            handle_get(&op, &path, out.as_deref()).await?;
        }
        Some(Commands::List { path, recursive }) => {
            handle_list(&op, &path, recursive).await?;
        }
        Some(Commands::Delete { path }) => {
            handle_delete(&op, &path).await?;
        }
        Some(Commands::Smoke) | None => {
            println!("--- Running Dropbox Prototype Smoke Test ---");
            handle_smoke_test(&op).await?;
        }
    }

    Ok(())
}
