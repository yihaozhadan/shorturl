use env_logger::Env;

pub fn init_logging() {
    let env = Env::default().filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env).init();
}
