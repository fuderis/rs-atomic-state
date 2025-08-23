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
