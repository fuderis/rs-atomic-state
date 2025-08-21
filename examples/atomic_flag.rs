extern crate atomic_state;  use atomic_state::{ Flag, flag };

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
