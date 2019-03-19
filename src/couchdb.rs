use std::str;
use std::io::{stdout, Write};
use base64::{encode, decode};
use roadrunner::RestClient;
use roadrunner::RestClientMethods;
use hyper::{StatusCode};

pub struct CouchDB{
    auth_basic: String,
    host: String,
    port: u32,
    proto: String,
}

impl CouchDB {
    pub fn new(user: String, password: String) -> CouchDB {
        CouchDB{
            auth_basic: encode(&format!("{}:{}", user, password)).to_string(),
            host: "localhost".to_string(),
            port: 5984,
            proto: "http".to_string(),
        }
    }

    pub fn cluster_setup(&self) -> Result<StatusCode, String> {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let data = format!(r#"{{"action":"enable_single_node", "bind_address":"0.0.0.0", "password":"{}", "port":"5984", "singlenode":"true", "username":"{}"}}"#,self.get_password(),self.get_username());
        let response = RestClient::post(&format!("{}/{}",self.get_base_url(), "_cluster_setup"))
            .authorization_basic(self.get_username(), self.get_password())
            .json_body_str(data)
            .execute_on(&mut core)
            .unwrap();

        Ok(*response.status())
    }    

    pub fn create_db(&self, db: String) -> Result<StatusCode, String>{
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let response = RestClient::put(&format!("{}/{}",self.get_base_url(), db))
            .authorization_basic(self.get_username(), self.get_password())
            .execute_on(&mut core)
            .unwrap();

        Ok(*response.status())
    } 

    pub fn get_auth(&self) -> &str {
        &self.auth_basic
    }

    pub fn get_base_url(&self) -> String {
        let base_url = format!("{}://{}:{}", self.get_protocol(), self.get_host(), self.get_port());
        base_url.to_owned()
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    fn get_base64_part(&self, idx: usize) -> Option<String> {
        match decode(&self.auth_basic) {
            Ok(decoded) => {
                let decoded = str::from_utf8(&decoded).unwrap().to_string();
                let parts: Vec<&str> = decoded.split(':').collect();

                Some(parts[idx].to_string())
            },
            Err(_err) => {
                //panic!("Warning: Bad encoded Authorization Header value: {}", encoded)
                 None
            }
        }
    }

    pub fn get_password(&self) -> String {
        let password = self.get_base64_part(1).unwrap();
        password.to_owned()
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }    

    pub fn get_protocol(&self) -> &str {
        &self.proto
    }

    pub fn get_username(&self) -> String {
        let username = self.get_base64_part(0).unwrap();
        username.to_owned()
    }    
    
    pub fn set_host(&mut self, host: String) -> bool {
        self.host = host;
        true
    }

    pub fn set_port(&mut self, port: u32) -> bool {
        self.port = port;
        true
    }

    pub fn set_protocol(&mut self, proto: String) -> bool {
        self.proto = proto;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_couchdb_new() {
        let couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert_eq!(couch.get_auth(), "Zm9vOmJhcg==");
        assert_eq!(couch.get_host(), "localhost");
        assert_eq!(couch.get_port(), 5984);
        assert_eq!(couch.get_protocol(), "http");
        assert_eq!(couch.get_base_url(), "http://localhost:5984".to_string())
    }

    #[test]
    fn test_cluster_setup() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        assert_eq!(couch.cluster_setup().unwrap(), StatusCode::Created);
    }    

    #[test]
    fn test_create_db_201() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        assert_eq!(couch.create_db("test".to_string()).unwrap(), StatusCode::Created);
    }

    #[test]
    fn test_create_db_401() {
        let couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert_eq!(couch.create_db("test2".to_string()).unwrap(), StatusCode::Unauthorized);
    }

    #[test]
    fn test_get_password() {
        let couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert_eq!(couch.get_password(), "bar".to_string());
    }

    #[test]
    fn test_get_username() {
        let couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert_eq!(couch.get_username(), "foo".to_string());
    }    

    #[test]
    fn test_set_host() {
        let mut couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert!(couch.set_host("127.0.0.1".to_string()));
        assert_eq!(couch.get_host(), "127.0.0.1".to_string());
    }

    #[test]
    fn test_set_port() {
        let mut couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert!(couch.set_port(8080));
        assert_eq!(couch.get_port(), 8080);
    }

    #[test]
    fn test_set_protocol() {
        let mut couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert!(couch.set_protocol("https".to_string()));
        assert_eq!(couch.get_protocol(), "https".to_string());
    }
}