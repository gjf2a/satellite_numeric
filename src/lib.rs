#[macro_use]
extern crate io_error;
pub mod methods;
pub mod operators;
pub mod pddl_parser;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
