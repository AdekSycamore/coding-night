use super::models::{CreateTodoInput, NewTodo, Todo, NewUser, CreateUserInput, User};
use super::schema::todos::dsl::*;
use super::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper::{FieldError, FieldResult};

const DEFAULT_TODO_DONE: bool = false;

pub struct Todos;

impl Todos {
    pub fn all_todos(conn: &mut PgConnection) -> FieldResult<Vec<Todo>> {
        let res = todos.load::<Todo>(conn);

        graphql_translate(res)
    }

    pub fn create_todo(conn: &mut PgConnection, new_todo: CreateTodoInput) -> FieldResult<Todo> {
        use super::schema::todos;

        let new_todo = NewTodo {
            task: &new_todo.task,
            done: &new_todo.done.unwrap_or(DEFAULT_TODO_DONE),
        };

        let res = diesel::insert_into(todos::table)
            .values(&new_todo)
            .get_result(conn);

        graphql_translate(res)
    }
}

pub struct Users;

impl User {
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
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}