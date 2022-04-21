use serde_json::Value as JsonValue;

#[derive(Debug)]
pub struct Client {
    pub host: String,
    pub username: String,
    pub password: String,
    pub scenarios_db: String,
}

impl Client {
    pub fn new(client: &JsonValue) -> Self {
        assert!(client.get("host").is_some(), "'host' not found for client");
        assert!(
            client.get("username").is_some(),
            "'username' not found for client"
        );
        assert!(
            client.get("password").is_some(),
            "'password' not found for client"
        );
        assert!(
            client.get("scenarios_db").is_some(),
            "'scenarios_db' not found for client"
        );

        Client {
            host: Client::value_from_key(client, "host"),
            username: Client::value_from_key(client, "username"),
            password: Client::value_from_key(client, "password"),
            scenarios_db: Client::value_from_key(client, "scenarios_db"),
        }
    }

    pub fn value_from_key(client: &JsonValue, raw_key: &str) -> String {
        client.get(raw_key).unwrap().as_str().unwrap().to_string()
    }
}
