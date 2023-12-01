use super::schema::*;
use juniper::GraphQLInputObject;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[juniper::graphql_object]
impl User {
    fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct Login {
    pub token: String,
}

#[juniper::graphql_object]
impl Login {
    fn token(&self) -> &str {
        self.token.as_str()
    }
}


#[derive(GraphQLInputObject)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub author: String,
    pub location: String,
    pub content: String,
}

#[juniper::graphql_object]
impl Post {
    fn id(&self) -> i32 {
        self.id
    }

    pub fn author(&self) -> &str {
        self.author.as_str()
    }

    pub fn location(&self) -> &str {
        self.location.as_str()
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub content: &'a str,
    pub location: &'a str,
    pub author: &'a str,
}

#[derive(GraphQLInputObject)]
pub struct CreatePostInput {
    pub content: String,
    pub location: String,
    pub author: String,
}
