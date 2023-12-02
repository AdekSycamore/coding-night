use super::schema::*;
use juniper::GraphQLInputObject;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub password: String,
    pub phone: String,
    pub region: String,
    pub firstname: String,
    pub lastname: String,
}

#[juniper::graphql_object]
impl User {
    fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }
    pub fn phone(&self) -> &str {
        self.phone.as_str()
    }
    pub fn region(&self) -> &str {
        self.region.as_str()
    }
    pub fn firstname(&self) -> &str {
        self.firstname.as_str()
    }
    pub fn lastname(&self) -> &str {
        self.lastname.as_str()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub phone: &'a str,
    pub region: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
}
#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub password: String,
    pub phone: String,
    pub region: String,
    pub firstname: String,
    pub lastname: String,
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
    pub title: String,
    pub link: String,
    pub maplink: String,
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

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn link(&self) -> &str{
        self.title.as_str()
    }
    pub fn maplink(&self) -> &str{
        self.maplink.as_str()
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub content: &'a str,
    pub location: &'a str,
    pub author: &'a str,
    pub title: &'a str,
    pub link: &'a str,
    pub maplink: &'a str,
}

#[derive(GraphQLInputObject)]
pub struct CreatePostInput {
    pub content: String,
    pub location: String,
    pub author: String,
    pub title: String,
    pub link: String,
    pub maplink: String,
}
