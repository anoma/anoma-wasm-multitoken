pub fn get_var_or_die(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Couldn't find {} in environment", key))
}
