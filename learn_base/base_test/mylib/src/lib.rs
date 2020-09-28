
pub mod animal;

#[cfg(test)]
mod tests {
    use crate::animal::cat;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn use_cat() {
        assert_eq!(true,cat::is_cat());
    }
}
