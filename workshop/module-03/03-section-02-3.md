#### Creating the module
>[couchdb.rs](https://github.com/dsietz/rust-daas/blob/master/src/couchdb.rs)

---

To create the module, create a new file named **_couchdb.rs_** in the **/src** directory.

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.

Add the `use` declarations at the top of the file.

```
use std::str;
use base64::{encode, decode};
use roadrunner::RestClient;
use roadrunner::RestClientMethods;
use hyper::{StatusCode};
use serde_json::value::{Value};
use super::daas::{DaaSDoc};
```

##### Tests

> NOTE: These tests require preexisting documents in CouchDB in the `test` database. You will need to create the `test` database and then create the following documents in the `test` database:
> 1. [doc-12345](https://github.com/dsietz/rust-daas/blob/master/tests/database/doc-12345.json)
> 2. [doc-upsert_existing_doc](https://github.com/dsietz/rust-daas/blob/master/tests/database/doc-upsert_existing_doc.json)
 
First create the tests for this module as a nested module at the bottom of the file.

```
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;

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

    #[ignore]
    #[test]
    fn test_create_db_201() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());

        match couch.create_db("test".to_string()).unwrap() {
            StatusCode::Created => assert!(true),
            StatusCode::PreconditionFailed => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_create_db_401() {
        let couch = CouchDB::new("foo".to_string(), "bar".to_string());
        assert_eq!(couch.create_db("test2".to_string()).unwrap(), StatusCode::Unauthorized);
    }

    #[test]
    fn test_create_doc_200() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        let src = "iStore".to_string();
        let mut rng = rand::thread_rng();
        let uid: usize = rng.gen_range(0, 1000000);
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
        let rslt = couch.create_doc("test".to_string(),doc);
               
        match rslt {
            Ok(_v) => {
                assert!(true);
            },
            Err(e) => {
                println!("Failed: {}",e);
                assert!(false);
            }
        }
    }    

    #[test]
    fn test_get_doc_by_id_200() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        
        match couch.get_doc_by_id("test".to_string(),"12345".to_string()) {
            Ok(doc) => {
                println!("DOC ID: {}", doc._id);
                assert_eq!(doc._id, "12345".to_string())
            },
            Err(_e) => assert!(false)
        }
    }

    #[test]
    fn test_get_doc_by_id_404() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        
        match couch.get_doc_by_id("test".to_string(),"bad-id".to_string()) {
            Ok(_v) => assert!(false),
            Err(_e) => assert!(true)
        }
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

    #[test]
    fn test_update_doc_200() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        let mut before = couch.get_doc_by_id("test".to_string(),"12345".to_string()).unwrap();
        let old_uid = before.source_uid.clone();
        let _rev = before._rev.clone().unwrap();
        let new_src_uid: usize = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        before.source_uid = new_src_uid;

        match couch.update_doc("test".to_string(), before) {
            Ok(_doc) => {
                match couch.get_doc_by_id("test".to_string(),"12345".to_string()) {
                    Ok(after) => assert_ne!(old_uid, after.source_uid),
                    _ => assert!(false)
                }
            },
            Err(_e) => assert!(false)
        }
    }   

    #[test]
    fn test_upsert_new_doc_200() {
        // create a new document
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        let src = "iStore".to_string();
        let mut rng = rand::thread_rng();
        let uid: usize = rng.gen_range(0, 1000000);
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
        let rslt = couch.upsert_doc("test".to_string(),doc);
               
        match rslt {
            Ok(_v) => {
                assert!(true);
            },
            Err(e) => {
                println!("Failed: {}",e);
                assert!(false);
            }
        }
    }    

    #[test]
    fn test_upsert_existing_doc_200() {
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        let mut before = couch.get_doc_by_id("test".to_string(),"upsert_existing_doc".to_string()).unwrap();
        let old_uid = before.source_uid.clone();
        let new_src_uid: usize = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        before.source_uid = new_src_uid;

        match couch.upsert_doc("test".to_string(), before) {
            Ok(_doc) => {
                match couch.get_doc_by_id("test".to_string(),"upsert_existing_doc".to_string()) {
                    Ok(after) => assert_ne!(old_uid, after.source_uid),
                    _ => assert!(false)
                }
            },
            Err(_e) => assert!(false)
        }
    }      

    #[test]
    fn test_upsert_existing_doc_without_rev_200() {
        // create a new document
        let couch = CouchDB::new("admin".to_string(), "password".to_string());
        let db = "test".to_string();
        let src = "iStore".to_string();
        let mut rng = rand::thread_rng();
        let uid: usize = rng.gen_range(0, 1000000);
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src.clone(), uid.clone(), cat.clone(), sub.clone(), auth.clone(), data.clone());

        match couch.create_doc(db.clone(), doc) {
            Ok(_rslt) => {
                let updt_doc = DaaSDoc::new(src, 6000, cat, sub, auth, data);
                let doc_id = updt_doc._id.clone();

                match couch.upsert_doc(db.clone(), updt_doc) {
                    Ok(_v) => {
                        let after = couch.get_doc_by_id(db.clone(), doc_id).unwrap();
                        assert_eq!(6000, after.source_uid);
                    },
                    Err(e) => {
                        println!("Failed: {}",e);
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                println!("Failed: {}",e);
                assert!(false);
            }
        }
    }             
}
```

##### Code

Just like in the daas module, we will follow an OOD and first create a structure for the CouchDB object.

```
pub struct CouchDB{
    auth_basic: String,
    host: String,
    port: u32,
    proto: String,
}
```

And then use `impl` to provide the CouchDB object with funcitonality. Notice the `new()` constructor function.

```
impl CouchDB {
    pub fn new(user: String, password: String) -> CouchDB {
        CouchDB{
            auth_basic: encode(&format!("{}:{}", user, password)).to_string(),
            host: "localhost".to_string(),
            port: 5984,
            proto: "http".to_string(),
        }
    }

    //this function doesn't work and setup needs to be done manually
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

    pub fn create_doc(&self, db: String, mut doc: DaaSDoc) -> Result<Value, String>{
        let json_doc = doc.serialize_without_rev();
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let response = RestClient::post(&format!("{}/{}",self.get_base_url(), db))
            .authorization_basic(self.get_username(), self.get_password())
            .json_body_str(json_doc)
            .execute_on(&mut core)
            .unwrap();

        match response.status() {
            StatusCode::Created => {
                Ok(response.content().as_value().unwrap())
            },
            _ => {
                println!("{}",response.content().as_ref_string());
                Err(format!("Wrong status code. Status {} was returned.",response.status()))
            },
        }
    }     

    pub fn get_auth(&self) -> &str {
        &self.auth_basic
    }

    pub fn get_base_url(&self) -> String {
        let base_url = format!("{}://{}:{}", self.get_protocol(), self.get_host(), self.get_port());
        base_url.to_owned()
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

    pub fn get_doc_by_id(&self, db: String, doc_id: String) -> Result<DaaSDoc, String>{
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let response = RestClient::get(&format!("{}/{}/{}",self.get_base_url(), db, doc_id))
            .authorization_basic(self.get_username(), self.get_password())
            .execute_on(&mut core)
            .unwrap();

        match response.status() {
            StatusCode::Ok => {
                Ok(DaaSDoc::from_serialized(response.content().as_ref_string()))
            },
            _ => Err(format!("Wrong status code. Status {} was returned.",response.status())),
        }
    } 

    pub fn get_host(&self) -> &str {
        &self.host
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
    
    pub fn query_view(&self, db: String, resource_path: String) -> Result<String, String>{
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let response = RestClient::get(&format!("{}/{}/{}",self.get_base_url(), db, resource_path))
            .authorization_basic(self.get_username(), self.get_password())
            .execute_on(&mut core)
            .unwrap();

        match response.status() {
            StatusCode::Ok => {
                Ok(response.content().as_ref_string().to_string())
            },
            _ => Err(format!("Wrong status code. Status {} was returned.",response.status())),
        }
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

    pub fn update_doc(&self, db: String, mut doc: DaaSDoc) -> Result<Value, String>{
        let json_doc = doc.serialize();
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let response = RestClient::put(&format!("{}/{}/{}",self.get_base_url(), db, doc._id))
            .authorization_basic(self.get_username(), self.get_password())
            .json_body_str(json_doc)
            .execute_on(&mut core)
            .unwrap();

        match response.status() {
            StatusCode::Created => {
                Ok(response.content().as_value().unwrap())
            },
            _ => {
                println!("{}",response.content().as_ref_string());
                Err(format!("Wrong status code. Status {} was returned.",response.status()))
            },
        }
    } 

    pub fn upsert_doc(&self, db: String, mut doc: DaaSDoc) -> Result<Value, String>{
        match self.get_doc_by_id(db.clone(), doc._id.clone()) {
            // exsisting document
            Ok(found) => {
                // add (or overwrite) the _rev to the doc
                doc._rev = found._rev;
                self.update_doc(db, doc)
            },
            // new document
            Err(_e) => {
                self.create_doc(db, doc)
            },
        }
    }     
}
```