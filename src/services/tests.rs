#[cfg(test)]
mod tests {
    use crate::services::structs::*;
    use std::collections::HashMap;

    fn host_init() -> Host {
        Host {
            host: String::from("127.0.0.1"),
            port: 9000,
            mode: ServerType::Server,
            password: String::from("sussy"),
            get_user_route: String::from("auth/user/<user_id>"),
            register_user_route: String::from("auth/user"),
        }
    }

    #[test]
    fn login() {
        let host = host_init();
        let mut login_data = HashMap::new();

        login_data.insert("identifier", "1234");
        login_data.insert("password", "sussy");
    }
}
