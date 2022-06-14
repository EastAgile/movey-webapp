use cucumber::{given, then, when};
use mainlib::settings::models::token::ApiToken;
use mainlib::test::DB_POOL;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
use mainlib::accounts::Account;

use super::super::world::TestWorld;

#[given("I have created the maximum number of allowed tokens")]
async fn maximum_tokens(_world: &mut TestWorld) {
    let max_token = std::env::var("MAX_TOKEN")
        .expect("MAX_TOKEN not set!")
        .parse::<i32>()
        .expect("MAX_TOKEN must be an integer");
    let account = Account::get(1, &DB_POOL).await.unwrap();
    for n in 0..max_token {
        ApiToken::insert(&account, &n.to_string(), &DB_POOL).await.unwrap();
    }
}
