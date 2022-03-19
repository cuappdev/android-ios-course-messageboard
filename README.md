# ios-course-messageboard

A simple server in rust for use in our iOS **and Android** course.

## Running (SKIP IF COURSE STUDENT)
Make sure you have [rust and cargo](https://www.rust-lang.org/tools/install) installed.
Simply run `cargo run` in the root of this project to run the server.
It currently binds to port 8080.

## Endpoints

All endpoints return 404 when incorrect post ids and/or unhashed usernames are provided.

All endpoints return `Post` objects:
```json
{
    "title": "String",
    "body": "String",
    "hashedPoster": "String",
    "timestamp": "ISO 8601 timestamp",
    "id": 0
}
```

### GET /posts/

Returns a list of posts. Optionally accepts a `poster` query parameter and returns only posts with that poster hash.

### GET /posts/{id}/
Returns the post with the given id, if one exists.

### POST /posts/
Creates a new post.

Body:
```json
{
    "title": "String",
    "body": "String",
    "poster": "unhashed username"
}
```

### POST /posts/{post_id}/
Updates the post with `post_id`, if one exists. The original poster username must be provided. Only the post body is editable.

Body:
```json
{
    "body": "String",
    "poster": "Unhashed username"
}
```

### DELETE /posts/{post_id}/
Updates the post with `post_id`, if one exists. The original poster username must be provided.

Body:
```json
{
    "poster": "Unhashed username"
}
```
