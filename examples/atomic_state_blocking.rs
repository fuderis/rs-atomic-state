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
