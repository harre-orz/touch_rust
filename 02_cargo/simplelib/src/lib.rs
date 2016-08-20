#![crate_type = "lib"]
#![crate_name = "simplelib"]
#[cfg(test)]
mod tests {
    use super::mylib;
    #[test]
    fn it_works() {
        assert_eq!(mylib::twice(2), 4);
    }
}

pub mod mylib {
    pub fn twice(x: i32) -> i32 {
        x * 2
    }
}