use cucumber::WorldInit;
use dotenv::dotenv;
use futures::{future::{ready}, FutureExt};

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
        .after(|_feature, _rule, _scenario, _world| {
            ready(TestDatabaseHelper::cleanup_test_database()).boxed_local()
        })
        .run("tests/features/").await;
}
