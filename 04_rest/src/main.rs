extern crate rustc_serialize;
#[macro_use]
extern crate nickel;
extern crate nickel_sqlite;

mod models;

use nickel::{
    Nickel,
    HttpRouter,
    JsonBody,
    Request,
    Response,
    Middleware,
    MiddlewareResult
};
use nickel_sqlite::{
    SqliteMiddleware,
    SqliteRequestExtensions
};
use rustc_serialize::json::{
    ToJson
};

use models::{
    Todo,
    TodoList,
    TodoForm,
    TodoUpdateForm
};

struct Logger;

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        println!("logging request from logger middleware: {:?}", req.origin.uri);
        res.next_middleware()
    }
}

fn main() {

    let db_url  = "resources/todo.sqlite";
    let mw      = SqliteMiddleware::new(&db_url).expect(
        "failed to connect to database");

    let mut server = Nickel::new();

    server.utilize(mw);
    server.utilize(Logger);

    server.get("/todo", middleware! { |request, response|
        let conn = request.db_conn().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, status FROM todo").unwrap();
        let todo_iter = stmt.query_map(&[], |row| {
            Todo {
                id   : row.get(0),
                title : row.get(1),
                description : row.get(2),
                status : row.get(3),
            }
        }).unwrap();

        let mut todolist = Vec::new();
        for todo in todo_iter {
            todolist.push(todo.unwrap());
        }

        let data = TodoList {
            data: todolist,
        };
        data.to_json()
    });

    server.get("/todo/:id", middleware! { |request, response|
        let id = request.param("id").unwrap();
        let conn = request.db_conn().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, title, description, status FROM todo WHERE id = ?"
            ).unwrap();
        let todo_iter = stmt.query_map(&[&id], |row| {
            Todo {
                id   : row.get(0),
                title : row.get(1),
                description : row.get(2),
                status : row.get(3),
            }
        }).unwrap();

        let mut todolist = Vec::new();
        for todo in todo_iter {
            todolist.push(todo.unwrap());
        }

        let data = TodoList {
            data: todolist,
        };
        data.to_json()
    });

    server.post("/todo", middleware! { |request, response|
        let todo = request.json_as::<TodoForm>().unwrap();
        let conn = request.db_conn().unwrap();
        match conn.execute("INSERT INTO todo (title, description, status)
                      VALUES ($1, $2, $3)",
                     &[&todo.title, &todo.description, &"0"]) {
            Ok(_)  => format!("You add id: {} title: {} desc: {}",
             conn.last_insert_rowid(), todo.title, todo.description),
            Err(e) => format!("You failed add todo {} ", e)
        }
    });

    server.delete("/todo/:id", middleware! { |request, response|
        let id = request.param("id").unwrap();
        let conn = request.db_conn().unwrap();
        match conn.execute("DELETE FROM todo WHERE id = $1",
            &[&id]) {
            Ok(_) => format!("delete {} success", id),
            Err(e) => format!("You failed delete todo {} ", e)
        }
    });

    server.put("/todo/:id", middleware! { |request, response|
        let todo = request.json_as::<TodoUpdateForm>().unwrap();
        let id = request.param("id").unwrap();
        let conn = request.db_conn().unwrap();
        match conn.execute("UPDATE todo SET title = $1,
                             description = $2,
                             status = $3 WHERE id = $4",
                     &[&todo.title, &todo.description, &todo.status, &id]) {
            Ok(_)  => format!("You update id: {} title: {} desc: {} status: {}",
             id, todo.title, todo.description, todo.status),
            Err(e) => format!("You failed edit todo {} ", e)
        }
    });

    server.listen("127.0.0.1:3000");
}