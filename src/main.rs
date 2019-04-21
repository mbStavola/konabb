#![feature(custom_attribute, decl_macro, futures_api)]
#![recursion_limit = "128"]
#[deny(rust_2018_idioms)]
#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use diesel::{prelude::MysqlConnection, r2d2::ConnectionManager as DieselConnectionManager};
use dotenv;
use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use sentry;
use sentry_actix::SentryMiddleware;

use crate::controllers::{board, post, thread, user};

mod controllers;
mod dao;
mod models;
mod schema;
mod util;

#[derive(Envconfig)]
struct Environment {
    #[envconfig(from = "SENTRY_DSN")]
    pub sentry_dsn: Option<String>,

    #[envconfig(from = "DATABASE_URL")]
    pub db_url: String,

    #[envconfig(from = "CACHE_URL")]
    pub cache_url: String,
}

fn main() {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let environment: Environment = Environment::init().unwrap();

    let _guard = if let Some(sentry_dsn) = environment.sentry_dsn {
        let _guard = sentry::init(sentry_dsn);
        sentry::integrations::panic::register_panic_handler();
        Some(_guard)
    } else {
        None
    };

    let db_manager = DieselConnectionManager::<MysqlConnection>::new(environment.db_url);
    let db_pool = Pool::builder()
        .build(db_manager)
        .expect("Failed to create db pool.");

    let cache_manager = RedisConnectionManager::new(environment.cache_url.as_ref()).expect("fwf");
    let cache_pool = Pool::builder()
        .build(cache_manager)
        .expect("Failed to create cache pool");

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(cache_pool.clone())
            .wrap(middleware::Logger::default())
            //.wrap(SentryMiddleware::new()) This needs to be updated to 1.0.0-alpha.3
            .service(
                web::scope("/user")
                    .route("/login", web::post().to(user::login))
                    .route("/{user_id}", web::get().to(user::get_user))
                    .route("", web::post().to(user::create_user))
                    .route("", web::put().to(user::update_user)),
            )
            .service(
                web::scope("/thread")
                    .route("", web::post().to(thread::create_thread))
                    .route("/{thread_id}", web::get().to(thread::get_thread))
                    .route("/{thread_id}", web::delete().to(thread::delete_thread)),
            )
            .service(
                web::scope("/post")
                    .route("", web::post().to(post::create_post))
                    .route("/{post_id}", web::put().to(post::update_post))
                    .route("/{post_id}", web::delete().to(post::delete_post)),
            )
            .service(web::scope("/board").route("", web::post().to(board::create_board)))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    });

    server.bind("127.0.0.1:8080").unwrap().run();
}
