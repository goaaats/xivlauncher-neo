pub mod game;

mod util;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
