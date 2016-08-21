extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct Todo {
    id: i32,
    title:  String,
    description: String,
}

fn main() {
    let mut server = Nickel::new();

    server.get("/todo", middleware! { |_, response|
        // let todo = request.json_as::<Todo>().unwrap();
        format!("you request todo list")
    });

    server.post("/todo", middleware! { |request, response|
        let todo = request.json_as::<Todo>().unwrap();
        format!("You send title: {} desc: {}", todo.title, todo.description)
    });

    server.listen("127.0.0.1:3000");
}