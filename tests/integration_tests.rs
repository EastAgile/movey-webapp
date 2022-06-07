use cucumber::WorldInit;
use dotenv::dotenv;
use futures::{future::{ready}, FutureExt, executor::block_on};

mod features;
use features::world::TestWorld;
use mainlib::test::TestDatabaseHelper;
use std::env;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    thread::Builder::new().name("test-server".to_string()).spawn(move || {
        let _ = actix_rt::System::new("test-server");
        env::set_var("PORT", "17002");
        block_on(mainlib::test_main());
    }).unwrap();

    // make sure the test server has time to boot up
    let twenty_seconds = Duration::from_secs(20);
    thread::sleep(twenty_seconds);

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
        }).run_and_exit("tests/features/").await;
}
