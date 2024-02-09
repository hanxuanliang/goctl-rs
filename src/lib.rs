pub mod parser;
pub mod cli;

mod common;
mod error;
mod service;
mod struct_ref;
mod token;

mod openapi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
