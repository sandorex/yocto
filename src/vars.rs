pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const FULL_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_GIT_DESCRIBE"), " (", env!("VERGEN_GIT_BRANCH"), ")");
