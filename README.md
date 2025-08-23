[![github]](https://github.com/fuderis/rs-atomic-state)&ensp;
[![crates-io]](https://crates.io/crates/atomic-state)&ensp;
[![docs-rs]](https://docs.rs/atomic-state)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

# Atomic State

It's a Rust library designed to simplify working with static asynchronous data that can be accessed safely and concurrently from any part of your program. It provides convenient abstractions for creating atomic flags and state objects with asynchronous setters, getters, and locking mechanisms.<br>

This library bridges the gap between synchronous static variables and asynchronous runtime environments, enabling seamless and safe manipulation of shared global state in asynchronous applications, such as those using Tokio or other async runtimes.


## Features:

* Define static atomic flags to represent simple boolean states.
* Create static asynchronous state wrappers around complex data structures.
* Perform asynchronous write operations with automatic synchronization.
* Access and modify shared state safely using async locks.
* Use ergonomic APIs for mapping and updating internal state asynchronously.

It's ideal for applications that require global configuration, feature flags, or any kind of shared state accessible across asynchronous tasks without compromising thread safety or requiring complicated boilerplate code.


## Examples:

### Atomic Flag:
```rust
use atomic_state::{ Flag, flag };

static IS_ACTIVE: Flag = flag!(false);

#[tokio::main]
async fn main() {
    assert_eq!(*IS_ACTIVE, false);
    assert!(IS_ACTIVE.is_false());

    IS_ACTIVE.set(true);
    assert_eq!(*IS_ACTIVE, true);

    IS_ACTIVE.swap(false).await;
    assert_eq!(*IS_ACTIVE, false);
}
```

### Atomic State:
```rust
use atomic_state::prelude::*;

static CONFIG: Lazy<AtomState<Config>> = lazy_state!(
    Config {
        count: 0,
    }
);

#[derive(Debug, Clone)]
pub struct Config {
    pub count: i32,
}

#[tokio::main]
async fn main() {
    CONFIG.set(Config { count: 10, }).await;
    assert_eq!(CONFIG.get().await.count, 10);

    (CONFIG.lock().await).count = 20;
    assert_eq!(CONFIG.get().await.count, 20);
}
```
#### With thread blocking:
```rust
use atomic_state::prelude::*;

static CONFIG: Lazy<AtomState<Config>> = lazy_state!(
    Config {
        count: 0,
    }
);

#[derive(Debug, Clone)]
pub struct Config {
    pub count: i32,
}

fn main() {
    CONFIG.block_set(Config { count: 10, });
    assert_eq!(CONFIG.block_get().count, 10);

    (CONFIG.block_lock()).count = 20;
    assert_eq!(CONFIG.block_get().count, 20);
}
```

## Feedback:

> This library distributed under the [MIT](https://github.com/fuderis/rs-atomic-state/blob/main/LICENSE.md) license.

You can contact me via GitHub or send a message to my telegram [@fuderis](https://t.me/fuderis).
This library is actively evolving, and your suggestions and feedback are always welcome!
