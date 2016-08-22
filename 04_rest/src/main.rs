extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
struct Todo {
    id: i32,
    title:  String,
    description: String,
    status: i32,
}

#[derive(RustcDecodable, RustcEncodable)]
struct TodoList {
    data: Vec<Todo>,
}

impl ToJson for Todo {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("title".to_string(), self.title.to_json());
        map.insert("description".to_string(), self.description.to_json());
        map.insert("status".to_string(), self.status.to_json());
        Json::Object(map)
    }
}

impl ToJson for TodoList {
    fn to_json(&self) -> Json {
        let mut vec = Vec::new();
        let mut outer = BTreeMap::new();
        for todo in &self.data {
            let mut map = BTreeMap::new();
            map.insert("id".to_string(), todo.id.to_json());
            map.insert("title".to_string(), todo.title.to_json());
            map.insert("description".to_string(), todo.description.to_json());
            map.insert("status".to_string(), todo.status.to_json());
            vec.push(map);
        }
        outer.insert("data".to_string(), vec.to_json());
        Json::Object(outer)
    }
}

fn main() {
    let mut server = Nickel::new();

    server.get("/todo", middleware! { |_, response|
        // let todo = request.json_as::<Todo>().unwrap();
        let todo1 = Todo {
            id: 1,
            title: "title1".to_string(),
            description: "desc1".to_string(),
            status: 1,
        };
        let todo2 = Todo {
            id: 2,
            title: "title2".to_string(),
            description: "desc2".to_string(),
            status: 0,
        };
        let todolist = vec![todo1, todo2];
        let data = TodoList {
            data: todolist,
        };
        data.to_json()
    });

    server.get("/todo/:id", middleware! { |_, response|
        // let todo = request.json_as::<Todo>().unwrap();
        let todo1 = Todo {
            id: 1,
            title: "title1".to_string(),
            description: "desc1".to_string(),
            status: 1,
        };
        todo1.to_json()
    });

    server.post("/todo", middleware! { |request, response|
        let todo = request.json_as::<Todo>().unwrap();
        format!("You send title: {} desc: {}", todo.title, todo.description)
    });

    server.delete("/todo/:id", middleware! { |request, response|
        let id = request.param("id").unwrap();
        format!("You delete id {}", id.to_string())
    });

    server.put("/todo/:id", middleware! { |request, response|
        let id = request.param("id").unwrap();
        format!("You change id {}", id.to_string())
    });

    server.listen("127.0.0.1:3000");
}