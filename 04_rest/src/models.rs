pub use self::models::Todo;
pub use self::models::TodoList;
pub use self::models::TodoUpdateForm;
pub use self::models::TodoForm;
pub mod models {
    use rustc_serialize::json::{
        Json,
        ToJson
    };
    use std::collections::BTreeMap;

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TodoForm {
        pub title:  String,
        pub description: String,
    }

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TodoUpdateForm {
        pub title:  String,
        pub description: String,
        pub status: i32,
    }

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct Todo {
        pub id: i32,
        pub title:  String,
        pub description: String,
        pub status: i32,
    }

    #[derive(RustcDecodable, RustcEncodable)]
    pub struct TodoList {
        pub data: Vec<Todo>,
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
}