#[cfg(test)]
mod test {
    use regex::Regex;

    #[test]
    fn t_main() {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        println!("Did our date match? {}", re.is_match("2014-01-01"));
    }
}
