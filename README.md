# AppSignal integration for Rust

Early stage proof of concept, use at your own risk.

## How to use

Add this crate to `Cargo.toml`:

```toml
[dependencies.appsignal]
git = "https://github.com/appsignal/appsignal-rs.git"
```

Configure and start:

```rust
set_var("APPSIGNAL_AGENT_PATH", "/path/to/agent");
set_var("APPSIGNAL_PUSH_API_KEY", "push-api-key");
set_var("APPSIGNAL_APP_NAME", "App name");
set_var("APPSIGNAL_ACTIVE", "true");
appsignal::start();
```

Track a transaction with an error:

```rust
let mut transaction = appsignal::Transaction::start("id", "background_job");
transaction.set_action("Worker#perform");
transaction.set_error(e); // Anything that can be printed with debug
transaction.finish();
transaction.complete();
```

Stop when the process exits:

```rust
appsignal::stop();
```


And that's it. Let us know on support at appsignal.com if you have any
questions or feedback.
