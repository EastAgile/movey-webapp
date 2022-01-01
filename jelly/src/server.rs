use std::env;
use std::sync::Arc;

use actix_session::CookieSession;
use actix_web::web::ServiceConfig;
use actix_web::{dev, middleware, web, App, HttpResponse, HttpServer};
use background_jobs::memory_storage::Storage;
use background_jobs::{create_server, WorkerConfig};
use sqlx::postgres::PgPoolOptions;

use crate::email::{Configurable, Email};
use crate::jobs::{JobState, DEFAULT_QUEUE};

/// This struct provides a slightly simpler way to write `main.rs` in
/// the root project, and forces more coupling to app-specific modules.
pub struct Server {
    apps: Vec<Box<dyn Fn(&mut ServiceConfig) + Send + Sync + 'static>>,
    jobs:
        Vec<Box<dyn Fn(WorkerConfig<JobState>) -> WorkerConfig<JobState> + Send + Sync + 'static>>,
}

impl Server {
    /// Creates a new Server struct to configure.
    pub fn new() -> Self {
        Self {
            apps: vec![],
            jobs: vec![],
        }
    }

    /// Registers a service.
    pub fn register_service<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut ServiceConfig) + Send + Sync + 'static,
    {
        self.apps.push(Box::new(handler));
        self
    }

    /// Registers jobs.
    pub fn register_jobs<F>(mut self, handler: F) -> Self
    where
        F: Fn(WorkerConfig<JobState>) -> WorkerConfig<JobState> + Send + Sync + 'static,
    {
        self.jobs.push(Box::new(handler));
        self
    }

    /// Consumes and then runs the server, with default settings that we
    /// generally want.
    pub async fn run(self) -> std::io::Result<dev::Server> {
        dotenv::dotenv().ok();
        pretty_env_logger::init();
        Email::check_conf();

        let bind = env::var("BIND_TO").expect("BIND_TO not set!");
        let _root_domain = env::var("DOMAIN").expect("DOMAIN not set!");

        #[cfg(feature = "production")]
        let domain = env::var("SESSIONID_DOMAIN").expect("SESSIONID_DOMAIN not set!");

        let key = env::var("SECRET_KEY").expect("SECRET_KEY not set!");

        let template_store = crate::templates::load();
        let templates = template_store.templates.clone();

        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL not set!");
        let pool = PgPoolOptions::new()
            .connect(&db_uri)
            .await
            .expect("Unable to connect to database!");

        let apps = Arc::new(self.apps);
        let jobs = Arc::new(self.jobs);

        let server = HttpServer::new(move || {
            // !production needs no domain set, because browsers.
            #[cfg(not(feature = "production"))]
            let session_storage = CookieSession::signed(key.as_bytes())
                .name("sessionid")
                .secure(false)
                .path("/");

            #[cfg(feature = "production")]
            let session_storage = CookieSession::signed(key.as_bytes())
                .name("sessionid")
                .path("/")
                .same_site(actix_web::cookie::SameSite::Lax)
                .domain(&domain)
                .secure(true);

            let mut app = App::new()
                .app_data(pool.clone())
                .app_data(templates.clone())
                .wrap(middleware::Logger::default())
                .wrap(session_storage)
                // Depending on your CORS needs, you may opt to change this
                // block. Up to you.
                .default_service(
                    web::resource("")
                        .route(web::get().to(crate::utils::not_found))
                        .route(web::head().to(HttpResponse::MethodNotAllowed))
                        .route(web::delete().to(HttpResponse::MethodNotAllowed))
                        .route(web::patch().to(HttpResponse::MethodNotAllowed))
                        .route(web::put().to(HttpResponse::MethodNotAllowed))
                        .route(web::post().to(HttpResponse::MethodNotAllowed)),
                )
                .configure(crate::utils::static_handler);

            for handler in apps.iter() {
                app = app.configure(|c| handler(c));
            }

            let storage = Storage::new();
            let queue = create_server(storage);
            let state = JobState::new("JobState", pool.clone(), templates.clone());
            let mut worker_config = WorkerConfig::new(move || state.clone());

            for handler in jobs.iter() {
                let x = handler.clone();
                worker_config = x(worker_config);
            }

            worker_config
                .set_worker_count(DEFAULT_QUEUE, 16)
                .start(queue.clone());

            app.app_data(web::Data::new(queue.clone()))
        })
        .backlog(8192)
        .shutdown_timeout(0)
        .workers(4)
        .bind(&bind)?
        .run();

        Ok(server)
    }
}
