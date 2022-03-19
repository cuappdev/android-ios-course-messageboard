//
//  Post.swift
//  
//
//  Created by Noah Pikielny on 3/19/22.
//

import FluentKit
import Vapor

final class Post: Model, Content {
    static var schema = "posts"
    
    @ID(key: .id)
    var id: UUID?
    
    @Field(key: "title")
    var title: String
    
    @Field(key: "body")
    var body: String
    
    @Field(key: "poster")
    var poster: String
    
    @Timestamp(key: "timeStamp", on: .create)
    var timeStamp: Date?
    
    init() {}
    
    init(id: UUID? = nil, title: String, body: String, poster: String, timeStamp: Date? = nil) {
        self.id = id
        self.title = title
        self.body = body
        self.poster = poster
        self.timeStamp = timeStamp
    }
    
}
