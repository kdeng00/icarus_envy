pub mod environment;
pub mod keys;

#[derive(Debug, Default, Clone)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

pub fn init_envvar(key: &str, value: &str) -> EnvVar {
    EnvVar {
        key: key.to_string(),
        value: value.to_string(),
    }
}
