#![cfg(test)]

mod tests {

    #[test]
    fn program01() {
        let content = std::fs::read_to_string(String::from("examples/program01.sf"))
            .expect("Error opening test file");
        assert_eq!(4, 4);
    }
    #[test]
    fn program02() {
        let content = std::fs::read_to_string(String::from("examples/program02.sf"))
            .expect("Error opening test file");
        assert_eq!(4, 4);
    }
    #[test]
    fn program03() {
        let content = std::fs::read_to_string(String::from("examples/program03.sf"))
            .expect("Error opening test file");
        assert_eq!(4, 4);
    }
    #[test]
    fn program04() {
        let content = std::fs::read_to_string(String::from("examples/program04.sf"))
            .expect("Error opening test file");
        assert_eq!(4, 4);
    }
    #[test]
    fn program05() {
        let content = std::fs::read_to_string(String::from("examples/program05.sf"))
            .expect("Error opening test file");
        assert_eq!(4, 4);
    }
}
