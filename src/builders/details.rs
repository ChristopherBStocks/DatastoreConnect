use crate::domain::Details;
use env_utils::libraries::dotenvy::var;

#[derive(Debug)]
#[non_exhaustive]
pub struct Builder {
    host: String,
    port: u16,
    user: String,
    password: String,
    name: String,
    secure: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(
            "localhost".to_owned(),
            3306,
            "zephyr".to_owned(),
            "zephyr".to_owned(),
            "zephyr".to_owned(),
            false,
        )
    }
}

impl Builder {
    #[must_use]
    pub fn new(
        host: String,
        port: u16,
        user: String,
        password: String,
        name: String,
        secure: bool,
    ) -> Self {
        Self {
            host,
            port,
            user,
            password,
            name,
            secure,
        }
    }

    #[must_use]
    pub fn from_env(prefix: &str) -> Self {
        let prefix = prefix.to_ascii_uppercase();
        Self {
            host: var(format!("{}_HOST", prefix)).unwrap_or_else(|_| "localhost".to_string()),
            port: var(format!("{}_PORT", prefix))
                .unwrap_or_else(|_| "3306".to_string())
                .parse()
                .unwrap_or(3306),
            user: var(format!("{}_USER", prefix)).unwrap_or_else(|_| "zephyr".to_string()),
            password: var(format!("{}_PASSWORD", prefix)).unwrap_or_else(|_| "zephyr".to_string()),
            name: var(format!("{}_NAME", prefix)).unwrap_or_else(|_| "zephyr".to_string()),
            secure: var(format!("{}_SECURE", prefix))
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_owned();
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_user(mut self, user: &str) -> Self {
        self.user = user.to_owned();
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_password(mut self, password: &str) -> Self {
        self.password = password.to_owned();
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    #[must_use]
    pub fn build(self) -> Details {
        Details {
            host: self.host,
            port: self.port,
            user: self.user,
            password: self.password,
            name: self.name,
            secure: self.secure,
        }
    }
}
