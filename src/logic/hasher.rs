use argon2::*;

lazy_static! {
    static ref TEMP_SALT: Vec<u8> = {
        let salt = std::env::var("SALT").unwrap();
        salt.into_bytes()
    };
    static ref SALT: &'static [u8] = &*TEMP_SALT;
    static ref TEMP_SECRET: Vec<u8> = {
        let secret = std::env::var("SECRET").unwrap();
        secret.into_bytes()
    };
    static ref SECRET: &'static [u8] = &*TEMP_SECRET;
}

#[derive(Clone)]
pub struct Hasher<'a> {
    config: Config<'a>,
}

impl<'a> Hasher<'a> {
    pub fn new(len: u32) -> Self {
        let config = Config {
            ad: &[],
            hash_length: len,
            lanes: 2,
            mem_cost: 4096,
            secret: *SECRET,
            thread_mode: ThreadMode::Parallel,
            time_cost: 7,
            variant: Variant::Argon2id,
            version: Version::default(),
        };

        Hasher { config }
    }

    pub fn hash<S: AsRef<str>>(&self, password: &S) -> String {
        argon2::hash_raw(password.as_ref().as_bytes(), *SALT, &self.config)
            .unwrap()
            .iter()
            .map(|&x| x as char)
            .collect::<String>()
            .replace("\n", "")
    }

    pub fn validate<T: AsRef<str>, S: AsRef<str>>(&self, hash: &T, password: &S) -> bool {
        hash.as_ref().to_string() == self.hash(password)
    }
}
