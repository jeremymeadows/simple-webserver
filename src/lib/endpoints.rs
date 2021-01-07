trait Endpoint {
    fn get(func: &dyn Fn());
}

impl Endpoint for &str {
    fn get(func: &dyn Fn()) {
    }
}

mod tests {
    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
