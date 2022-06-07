use cucumber::WorldInit;
use dotenv::dotenv;
use futures::{future::{ready}, FutureExt, executor::block_on};

mod features;
use features::world::TestWorld;
use mainlib::test::TestDatabaseHelper;

#[tokio::main]
async fn main() {
    TestWorld::cucumber()
        .before(|_feature, _rule, _scenario, _world| {
            dotenv().ok();
            ready(TestDatabaseHelper::create_test_database()).boxed_local()
        })
        .after(|_feature, _rule, _scenario, world| {
            if let Some(w) = world {
                block_on(w.close_browser());
            }
            ready(TestDatabaseHelper::cleanup_test_database()).boxed_local()
        })
        .run("tests/features/").await;
}
