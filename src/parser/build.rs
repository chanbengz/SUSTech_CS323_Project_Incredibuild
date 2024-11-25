fn main() {
    lalrpop::Configuration::new()
        .log_info()
        .process_current_dir().unwrap();
}
