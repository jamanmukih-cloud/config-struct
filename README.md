# Config Struct 🎛️

Type-safe configuration from env, files, and CLI.

## Sources (priority order)

1. CLI arguments
2. Environment variables
3. Config file (TOML/YAML)
4. Defaults

## Features

- **Validation**: Custom validators
- **Defaults**: Type-safe defaults
- **Secrets**: Auto-redact in logs
- **Hot-reload**: File watcher

## Quick Start

```rust
#[derive(Config)]
struct AppConfig {
    #[env("HOST")]
    host: String,
    #[env("PORT", default = "8080")]
    port: u16,
}
```

## License

MIT