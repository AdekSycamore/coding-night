use super::models::{ NewUser, CreateUserInput, User, LoginInput, Login, Post, NewPost, CreatePostInput};
use super::schema::users::dsl::*;
use super::schema::posts::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

pub struct Users;

impl Users {
    pub fn all_users(conn: &mut PgConnection) -> FieldResult<Vec<User>> {
        let res = users.load::<User>(conn);

        graphql_translate(res)
    }

    pub fn create_user(conn: &mut PgConnection, new_user: CreateUserInput) -> FieldResult<User> {
        use super::schema::users;
        let new_user = NewUser {
            username: &new_user.username,
            password: &new_user.password,
        };

        let res = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn login(conn: &mut PgConnection, input: LoginInput) -> FieldResult<Login> {

        let existing_user = users
            .filter(username.eq(input.username))
            .filter(password.eq(input.password))
            .get_result::<User>(conn);

        let x = Login {token: String::from("someusername123")};
        
        match existing_user {
           Ok(_registered_user) => Ok(x),
           Err(e) => FieldResult::Err(FieldError::from(e)),
        }
    }
}

pub struct Posts;
impl Posts {
    pub fn all_posts(conn: &mut PgConnection) -> FieldResult<Vec<Post>> {
        let res = posts.load::<Post>(conn);

        graphql_translate(res)
    }

    pub fn create_post(conn: &mut PgConnection, new_post: CreatePostInput) -> FieldResult<Post> {
        use super::schema::posts;

        let new_post = NewPost {
            content: &new_post.content,
            location: &new_post.location,
            author: &new_post.location,
        };

        let res = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn);

        graphql_translate(res)
    }
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}