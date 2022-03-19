//
//  File.swift
//  
//
//  Created by Noah Pikielny on 3/19/22.
//

import Vapor
import Fluent

struct PostController: RouteCollection {
    func boot(routes: RoutesBuilder) throws {
        let posts = routes.grouped("posts")
        posts.get(use: getAllPosts)
        posts.get(":postID", use: getPost)
        
        posts.post(use: create)
        
        posts.put(":postID", ":poster", use: update)
        
        posts.delete(":postID", use: delete)
        posts.delete("reset", ":username", ":password", use: reset)
    }
    
    func getAllPosts(req: Request) -> EventLoopFuture<[Post]> {
        Post.query(on: req.db).all()
    }
    
    func getPost(req: Request) throws -> EventLoopFuture<Post> {
        return Post.find(req.parameters.get("postID"), on: req.db)
            .unwrap(or: Abort(.notFound))
    }
    
    func create(req: Request) throws -> EventLoopFuture<Post> {
        let post = try req.content.decode(Post.self)
        let _ = post.save(on: req.db)
        return Post.find(post.id, on: req.db)
            .unwrap(or: Abort(.notFound))
    }
    
    func update(req: Request) throws -> EventLoopFuture<HTTPStatus> {
        let newPost = try req.content.decode(Post.self)
        return Post.find(req.parameters.get("postID"), on: req.db)
            .unwrap(or: Abort(.notFound))
            .flatMap {
                if $0.poster != newPost.poster {
                    return $0.update(on: req.db).transform(to: .notFound)
                }
                
                $0.body = newPost.body
                return $0.update(on: req.db).transform(to: .ok)
            }
    }
    
    func reset(req: Request) throws -> EventLoopFuture<HTTPStatus> {
        if req.parameters.get("username") == "iOS Instructors" && req.parameters.get("password") == "I Love Walker White" { // change these to secrets
            return Post.query(on: req.db).all()
                .map { $0.delete(on: req.db) }
                .transform(to: .ok)
        } else {
            throw Abort(.unauthorized)
        }
    }
    
    func delete(req: Request) throws -> EventLoopFuture<HTTPStatus> {
        return Post.find(req.parameters.get("postID"), on: req.db)
            .unwrap(or: Abort(.notFound))
            .flatMap { $0.delete(on: req.db) }
            .transform(to: .ok)
    }
}
