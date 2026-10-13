# OpenDAL Dropbox Prototype

Throwaway spike to validate Apache OpenDAL (`0.59.3`) operations against Dropbox:
- **upload**: upload inline text or a local file
- **get**: fetch file content to stdout or write to a local path
- **list**: list entries (flat or recursive)
- **delete**: remove a remote file
- **smoke**: automated round-trip probe (upload -> verify get -> check list -> delete)

## Supplying Credentials

You can supply your Dropbox developer access token or key in any of the following ways:

### Option 1: Interactive prompt
Run any command without arguments or environment variables. The CLI will prompt:
```bash
cargo run -- list
# Prompts: Please enter your Dropbox Access Token / Key:
```

### Option 2: CLI argument
```bash
cargo run -- --token "sl.u.YOUR_ACCESS_TOKEN" list
# or using -k / alias --key:
cargo run -- -k "sl.u.YOUR_ACCESS_TOKEN" smoke
```

### Option 3: Environment variable
```bash
export DROPBOX_TOKEN="sl.u.YOUR_ACCESS_TOKEN"
# (or DROPBOX_ACCESS_TOKEN, DROPBOX_KEY, or in .env file)
cargo run -- list
```

### Option 4: OAuth Refresh Token Flow (Long-Lived)
```bash
export DROPBOX_REFRESH_TOKEN="<refresh_token>"
export DROPBOX_CLIENT_ID="<client_id>"
export DROPBOX_CLIENT_SECRET="<client_secret>"
cargo run -- list
```

---

## Usage Examples

### 1. End-to-End Smoke Test
Uploads a probe file, validates content integrity, checks listing, and cleans it up:
```bash
cargo run -- smoke
# or simply:
cargo run
```

### 2. Upload
```bash
# Upload inline text
cargo run -- upload test.txt --content "Hello from OpenDAL"

# Upload a local file
cargo run -- upload remote_backup.tar.gz --file ./local_file.tar.gz
```

### 3. Get / Download
```bash
# Print remote content to stdout
cargo run -- get test.txt

# Save to a local file
cargo run -- get test.txt --out ./downloaded_test.txt
```

### 4. List
```bash
# List root folder
cargo run -- list

# List specific directory recursively
cargo run -- list backups/ --recursive
```

### 5. Delete
```bash
cargo run -- delete test.txt
```
