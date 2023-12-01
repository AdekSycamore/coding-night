use super::models::{CreateTodoInput, NewTodo, Todo, NewUser, CreateUserInput, User, LoginInput, Login};
use super::schema::todos::dsl::*;
use super::schema::users::dsl::*;
use super::utils::{hash, verify, create_jwt};
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

impl Users {
    pub fn all_users(conn: &mut PgConnection) -> FieldResult<Vec<User>> {
        let res = users.load::<User>(conn);

        graphql_translate(res)
    }

    pub fn create_user(conn: &mut PgConnection, new_user: CreateUserInput) -> FieldResult<User> {
        use super::schema::users;

        let hashed_password: Result<String, easy_password::bcrypt::PasswordError> = hash(&new_user.password);   

        let mut passwordx = String::new();

        match hashed_password {
            Ok(hash) => {passwordx = hash.to_string()},
            Err(_) => return FieldResult::Err(FieldError::from("error")),
        }

        let new_user = NewUser {
            username: &new_user.username,
            password: &passwordx,
        };

        let res = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn login(conn: &mut PgConnection, input: LoginInput) -> FieldResult<Login> {
        use super::schema::users;

        let existing_user: Result<User, diesel::result::Error> = users
            .filter(username.eq(input.username))
            .get_result::<User>(conn);

        match existing_user {
           Ok(registered) => match verify(&registered.password, &input.password) {
               Ok(valid) => {
                if valid { 
                     match create_jwt(&registered.username) {
                        Ok(t) => Ok(Login {token: t}),
                        Err(_) => FieldResult::Err(FieldError::from("cannot create token")) 
                    } 
                } else { 
                    return FieldResult::Err(FieldError::from("inncorrect")) 
                }
            },
            Err(_) => FieldResult::Err(FieldError::from("inncorrect"))
           },
           Err(_) => FieldResult::Err(FieldError::from("inncorrect")),
        }
    }
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}