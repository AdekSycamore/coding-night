use super::context::GraphQLContext;
use super::data::Todos;
use super::models::{CreateTodoInput, Todo};

use diesel::pg::PgConnection;
use juniper::{EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    pub fn all_todos(context: &GraphQLContext) -> FieldResult<Vec<Todo>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Todos::all_todos(conn)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = GraphQLContext)]
impl MutationRoot {
    pub fn create_todo(context: &GraphQLContext, input: CreateTodoInput) -> FieldResult<Todo> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        Todos::create_todo(conn, input)
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
