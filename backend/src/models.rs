use super::schema::*;
use juniper::GraphQLInputObject;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub done: bool,
}

#[juniper::graphql_object]
impl Todo {
    fn id(&self) -> i32 {
        self.id
    }

    pub fn task(&self) -> &str {
        self.task.as_str()
    }

    fn done(&self) -> bool {
        self.done
    }
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub task: &'a str,
    pub done: &'a bool,
}

#[derive(GraphQLInputObject)]
pub struct CreateTodoInput {
    pub task: String,
    pub done: Option<bool>,
}

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


