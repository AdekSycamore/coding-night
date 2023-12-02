use crate::models::{Post, CreatePostInput};
use serde::{Deserialize, Serialize};
use serde_json::de;

use super::context::GraphQLContext;
use super::data::{Posts, Users};
use super::models::{User, CreateUserInput, LoginInput, Login};

use diesel::pg::PgConnection;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, Header};
use juniper::{EmptySubscription, FieldResult, RootNode, FieldError};
use std::collections::HashMap;
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

        let token_message =  decode::<HashMap<String, serde_json::Value>>(&context.token,  &DecodingKey::from_secret("secret".as_ref()), &Validation::new(Algorithm::HS512)).unwrap();

        println!("{:?}", token_message);

        Ok(Login { token: String::from("fdfd") })
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
