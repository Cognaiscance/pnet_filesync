# pnet_filesync

Hybrid filesync app for pNet. See [description.md](description.md).

```bash
# Terminal 1 — pNet
PNET_AUTO_APPROVE_APPS=1 cargo run --manifest-path ../pNet/Cargo.toml

# Terminal 2 — agent + web
cargo run
```

Open `http://127.0.0.1:8777/`, sign in, Home → **Filesync** (`/apps/filesync/`).
Default folder: `~/pnet-filesync`.
