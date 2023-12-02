use crate::models::{Post, CreatePostInput};
use serde::{Deserialize, Serialize};

use super::context::GraphQLContext;
use super::data::{Posts, Users};
use super::models::{User, CreateUserInput, LoginInput, Login};

use diesel::pg::PgConnection;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, Header};
use juniper::{EmptySubscription, FieldResult, RootNode, FieldError};

pub struct QueryRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::all_users(conn)
    }

    pub fn login(context: &GraphQLContext, input: LoginInput) -> FieldResult<Login> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::login(conn, input)
    }

    pub fn all_posts(context: &GraphQLContext) -> FieldResult<Vec<Post>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Posts::all_posts(conn)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl MutationRoot {
     pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::create_user(conn, input)
    }

    pub fn create_post(context: &GraphQLContext, input: CreatePostInput) -> FieldResult<Login> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        let decoded_username: Result<TokenData<String>, jsonwebtoken::errors::Error> = decode::<String>(&context.token, &DecodingKey::from_secret(b"secret"), &Validation::new(Algorithm::HS512));

        match decoded_username {
            Ok(u) =>  Ok(Login {token: u.claims}),
            Err(e) => FieldResult::Err(FieldError::from(e)),
        }
        
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::<GraphQLContext>::new(),
    )
}
