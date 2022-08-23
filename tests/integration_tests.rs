use cucumber::WorldInit;
use dotenv::dotenv;
use futures::{executor::block_on, future::ready, FutureExt};

mod features;
use features::world::TestWorld;
use mainlib::test::TestDatabaseHelper;
use std::env;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    dotenv().ok();
    {
        TestDatabaseHelper::create_test_database();
        ready(())
    }
    .boxed_local();
    tokio::spawn(async {
        let _ = actix_rt::System::new("test-server");
        env::set_var("PORT", "17002");
        mainlib::main().await.unwrap();
    });

    // make sure the test server has time to boot up
    //TODO: optimize this value
    sleep(Duration::from_secs(30)).await;
    TestWorld::cucumber()
        .before(|_feature, _rule, _scenario, _world| {
            dotenv().ok();
            {
                TestDatabaseHelper::create_test_database();
                ready(())
            }
            .boxed_local()
        })
        .after(|_feature, _rule, _scenario, world| {
            if let Some(w) = world {
                block_on(w.close_browser());
            }
            {
                TestDatabaseHelper::cleanup_test_database();
                ready(())
            }
            .boxed_local()
        })
        .run_and_exit("tests/features/")
        .await;
}
