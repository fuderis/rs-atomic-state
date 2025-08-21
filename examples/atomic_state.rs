extern crate atomic_state;  use atomic_state::{ State, state };

static CONFIG: State<Config> = state!(
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
    assert_eq!(CONFIG.get().count, 0);

    CONFIG.set(Config { count: 10, }).await;
    assert_eq!(CONFIG.get().count, 10);

    (*CONFIG.lock().await).count = 20;
    assert_eq!(CONFIG.get().count, 20);

    CONFIG.map(|cfg| { cfg.count = 30; }).await;
    assert_eq!(CONFIG.get().count, 30);
}
