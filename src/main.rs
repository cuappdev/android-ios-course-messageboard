use actix_web::http::StatusCode;
use actix_web::{delete, get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Utc};
use hex::ToHex;
use linked_hash_map::LinkedHashMap;
use ring::digest::{digest, SHA256};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

const MAX_POSTS: usize = 1000;

fn hash(input: String) -> String {
    digest(&SHA256, input.as_bytes()).encode_hex()
}

struct State {
    id_counter: usize,
    posts: LinkedHashMap<usize, Post>,
    id_start: usize,
}

impl State {
    fn new() -> RwLock<Self> {
        RwLock::new(Self {
            id_counter: 0,
            posts: LinkedHashMap::new(),
            id_start: 0,
        })
    }
}

type AppData = web::Data<RwLock<State>>;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Post {
    title: String,
    body: String,
    hashed_poster: String,
    timestamp: DateTime<Utc>,
    id: usize,
}

#[derive(Deserialize, Serialize)]
struct CreatePost {
    title: String,
    body: String,
    poster: String,
}

#[derive(Deserialize, Serialize)]
struct UpdatePost {
    body: String,
    poster: String,
}

#[derive(Deserialize)]
struct DeletePost {
    poster: String,
}

#[derive(Deserialize)]
struct ByPoster {
    poster: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[get("/posts/")]
async fn get_posts(data: AppData, web::Query(poster_hash): web::Query<ByPoster>) -> impl Responder {
    match data.read() {
        Ok(data) => {
            if let Some(ph) = poster_hash.poster {
                HttpResponse::Ok().json2(
                    &data
                        .posts
                        .values()
                        .filter(|v| v.hashed_poster == ph)
                        .collect::<Vec<_>>(),
                )
            } else {
                HttpResponse::Ok().json2(&data.posts.values().collect::<Vec<_>>())
            }
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[get("/posts/{post_id}/")]
async fn get_post(web::Path(post_id): web::Path<usize>, data: AppData) -> impl Responder {
    match data.read() {
        Ok(data) => match data.posts.get(&post_id) {
            Some(post) => HttpResponse::Ok().json2(post),
            None => HttpResponse::NotFound().body("No post with that id."),
        },
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[post("/posts/")]
async fn create_post(web::Json(new_post): web::Json<CreatePost>, data: AppData) -> impl Responder {
    match data.write() {
        Ok(mut data) => {
            let id = data.id_counter;
            data.id_counter += 1;

            let post = Post {
                id,
                title: new_post.title,
                body: new_post.body,
                hashed_poster: hash(new_post.poster),
                timestamp: Utc::now(),
            };

            data.posts.insert(id, post.clone());
            while data.posts.len() > MAX_POSTS {
                let id_start = data.id_start;
                data.posts.remove(&id_start);
                data.id_start += 1;
            }
            HttpResponse::build(StatusCode::OK).json(post)
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[post("/posts/{post_id}/")]
async fn update_post(
    web::Path(post_id): web::Path<usize>,
    web::Json(update): web::Json<UpdatePost>,
    data: AppData,
) -> impl Responder {
    match data.write() {
        Ok(mut data) => match data.posts.get_mut(&post_id) {
            Some(post) if post.hashed_poster == hash(update.poster) => {
                post.body = update.body;
                HttpResponse::build(StatusCode::OK).json(post)
            }
            _ => HttpResponse::NotFound().body("No post with that id and poster."),
        },
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[delete("/posts/{post_id}/")]
async fn delete_post(
    web::Path(post_id): web::Path<usize>,
    web::Json(DeletePost { poster }): web::Json<DeletePost>,
    data: AppData,
) -> impl Responder {
    match data.write() {
        Ok(mut data) => {
            if let Some(true) = data
                .posts
                .get(&post_id)
                .map(|p| p.hashed_poster == hash(poster))
            {
                match data.posts.remove(&post_id) {
                    Some(post) => HttpResponse::build(StatusCode::OK).json(post),
                    None => HttpResponse::NotFound().body("No post with that id."),
                }
            } else {
                HttpResponse::NotFound().body("No post with that id and poster.")
            }
        }
        Err(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data: AppData = web::Data::new(State::new());

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::NormalizePath::default())
            .service(index)
            .service(get_post)
            .service(get_posts)
            .service(create_post)
            .service(delete_post)
            .service(update_post)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
