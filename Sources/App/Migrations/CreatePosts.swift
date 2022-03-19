//
//  CreatePost.swift
//  
//
//  Created by Noah Pikielny on 3/19/22.
//

import FluentKit

struct CreatePosts: AsyncMigration {
    func prepare(on database: Database) async throws {
        try await database.schema(Post.schema)
            .id()
            .field("title", .string, .required)
            .field("body", .string, .required)
            .field("poster", .string, .required)
            .field("timeStamp", .datetime, .required)
            .create()
    }
    
    func revert(on database: Database) async throws {
        try await database.schema(Post.schema).delete()
    }
}
