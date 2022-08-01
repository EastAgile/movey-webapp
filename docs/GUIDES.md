# Developer's guide

Documentation for developers working on Movey. For Move developers, please check out [`MOVE_DEVELOPER.md`](./MOVE_DEVELOPER.md).

## Getting Started

- This project is an extension of [Jelly](https://github.com/secretkeysio/jelly-actix-web-starter), which itself is an extension of [actix-web](https://actix.rs/), so you should see that some features are built around what is already available in Jelly (check out [JELLY_FEATURES.md](./docs/JELLY_FEATURES.md) for more details).
- To get started, clone this repository and change into the repository's directory from your terminal (currently this is our proprietary repo, we can change this later when this goes opensource).

```
git clone https://github.com/EastAgile/ea-movey.git
cd ea-movey
```

### Working on the Frontend

Our frontend is rendered on the server with basic stack:

- Html, using [Tera](https://github.com/Keats/tera) templating language
- Css, compiled from Scss
- JavaScript, with the help of jQuery and other libraries

Once you get the backend running, the frontend should not require any seperate operations other than having a scss compiler running in the background. You can keep the scss compiler running using `cargo-watch`.

```
cargo-watch -d 2.5 -s "sass --load-path . static/css"
```

### Working on the Backend

- Ensure you have Postgresql installed.
- Install [`diesel-cli`](https://diesel.rs/), with:

```
cargo install diesel_cli --no-default-features --features postgres
```

- Copy `.env.example` to `.env` and make modifications to run your settings.
- Run the migrations:

```
diesel setup
diesel migration run
```

- Run the server:

```
cargo run
```

If you're ready to push a release build, you probably want to run:

```
cargo build --release --no-default-features --features production
```

For configuring email dispatch, see the README in `email_templates`.

## Running Movey with Docker

All this process can be automated if you have [`Docker`](https://www.docker.com/) installed. Run `docker compose up` to get everything up and running effortlessly.

## Testing

For unit tests, execute

```
cargo test_unit
```

to run all the tests, or

```
cargo test --lib --package=movey_app --features=test -- <insert test name or name prefix> --test-threads=1
```

to run specific tests.

For integration tests, we use [Cucumber](https://github.com/cucumber-rs/cucumber) and a [Selenium](https://github.com/stevepryde/thirtyfour) library for Rust. To run integration tests, you must have [`chromedriver`](https://chromedriver.chromium.org/) installed. Then you can run

```
chromedriver --port=4444
```

and run

```
cargo test_integration
```

to run all the tests, or

```
cargo test --package=movey_app --features=test --test=integration_tests -- -t=@wip --concurrency=1
```

to run tests that has a `@wip` tag preceeding them.

## Staging

To deploy to [our staging website](https://movey-app-staging.herokuapp.com), please follow this procedure:

- Login to Heroku and set up `staging` to track https://git.heroku.com/movey-app-staging.git
- Checkout to the latest `development` branch
- Merge your branch and resolve any emerged conflicts
- Push development to origin
- Push development to staging at `master` branch

```
git push staging development:master
```

## Crawling

We have an environment variable called `CRAWLING` to control if we want to continue crawling for new packages. The crawler will begin to run whenever the server restarts.

Currently we are using Github Search API to find new packages, and we haven't got any way to crawl packages that are located on a specific chain.

## Credentials

Please [contact us](https://www.movey.net/contact) or email us directly at `movey@eastagile.com` for our project credentials.
