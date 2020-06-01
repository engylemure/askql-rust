pub mod resource;
pub mod resources;
pub mod run;
pub mod r#type;
pub mod typed;

use r#type::Type;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
