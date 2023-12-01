use super::context::GraphQLContext;
use super::data::{Todos, Users};
use super::models::{CreateTodoInput, Todo, User, CreateUserInput, LoginInput, Login};

use diesel::pg::PgConnection;
use juniper::{EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Todos::all_todos(conn)
    }

    pub fn all_users(context: &GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::all_users(conn)
    }

    pub fn login(context: &GraphQLContext, input: LoginInput) -> FieldResult<Login> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::login(conn, input)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl MutationRoot {
    pub fn create_todo(context: &GraphQLContext, input: CreateTodoInput) -> FieldResult<Todo> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Todos::create_todo(conn, input)
    }

    pub fn create_user(context: &GraphQLContext, input: CreateUserInput) -> FieldResult<User> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Users::create_user(conn, input)
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
