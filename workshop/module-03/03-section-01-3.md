#### Creating the module
>[dass.rs](https://github.com/dsietz/rust-daas/blob/master/src/daas.rs)

---

To create the module, create a new file named **_daas.rs_** in the **/src** directory.

> Now is a good time to rerun the `cargo test` command to ensure all your tests still pass.

Add the `use` declarations at the top of the file.

```
use super::*;
use serde_json::value::*;
```

In order to reduce _copy/paste_ throughout the code, we create a static VARIABLE just after the `use` declaration.

```
static DELIMITER: &str = "|";
```

##### Tests

First create the tests for this module as a nested module at the bottom of the file.

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ob_ok() {
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let _doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
        
        assert!(true);
    }

    #[test]
    fn test_doc_id_ok() {
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let id = format!("{}|{}|{}|{}",cat, sub, src, uid).to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
        
        assert_eq!(doc._id, id);
    }

    #[test]
    fn test_doc_rev_empty() {
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
        
        assert!(doc._rev.is_none());
    }

    #[test]
    fn test_doc_attributes_ok() {
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
        
        assert_eq!(doc.source_name, src);
        assert_eq!(doc.source_uid, uid);
        assert_eq!(doc.category, cat);
        assert_eq!(doc.subcategory, sub);
        assert_eq!(doc.process_ind, false);
    } 

    #[test]
    fn test_doc_data_ok() {
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
        
        assert_eq!(doc.data_obj.get("status").unwrap(), "new");
    }     

    #[test]
    fn test_from_serialize(){
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let id = format!("{}|{}|{}|{}",cat, sub, src, uid).to_string();
        let serialized = r#"{"_id":"order|clothing|iStore|5000","_rev":null,"source_name":"iStore","source_uid":5000,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"last_updated":1553988607,"data_obj":{"status":"new"}}"#;
        let doc = DaaSDoc::from_serialized(&serialized);
  	
        assert_eq!(doc._id, id);
        assert!(doc._rev.is_none());
        assert_eq!(doc.source_name, src);
        assert_eq!(doc.source_uid, uid);
        assert_eq!(doc.category, cat);
        assert_eq!(doc.subcategory, sub);
        assert_eq!(doc.author, auth);
        assert_eq!(doc.process_ind, false);
		assert_eq!(doc.data_obj.get("status").unwrap(), "new");
    }         

    #[ignore]
    #[test]
    fn test_serialize(){
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let mut doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth, data);

        let serialized = r#"{"_id":"order|clothing|iStore|5000","_rev":null,"source_name":"iStore","source_uid":5000,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"data_obj":{"status":"new"}}"#;
    	
		assert_eq!(doc.serialize(), serialized);
    }    

    #[ignore]
    #[test]
    fn test_serialize_without_rev(){
        let src = "iStore".to_string();
        let uid = 5000;
        let cat = "order".to_string();
        let sub = "clothing".to_string();
        let auth = "istore_app".to_string();
        let data = json!({
            "status": "new"
        });
        let mut doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
        let no_rev = r#"{"_id":"order|clothing|iStore|5000","source_name":"iStore","source_uid":5000,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"data_obj":{"status":"new"}}"#;
		
        assert_eq!(doc.serialize_without_rev(), no_rev.to_string());
    }   
}
```

##### Code

A critical step in the DaaS pattern design is the format used for the Unique Identifiers of the DaaS data objects. To handle this, we create a function in the module - `make_id()`.

```
pub fn make_id(cat: String, subcat: String, src_name: String, src_uid: usize) -> String {
    format!("{}{}{}{}{}{}{}",cat, DELIMITER, subcat, DELIMITER, src_name, DELIMITER, src_uid).to_string()
}    
```

The Rust language not only supports modules, but also more OOD principles by utilizing Structures and Implementations. To do this, we first deifne the `DaaS Document` and `DaaS Document without a Revision` objects.

```
/// Represents an existing DaaS document (after it has been saved and assigned a _rev value)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaaSDoc{
    /// The unique identifier
    pub _id: String,
    /// The revision number
    pub _rev: Option<String>,
    /// The name of the data source
    pub source_name: String,
    /// The unique identifier that the data source provides
    pub source_uid: usize,
    /// The name of the category (e.g.: order)
    pub category: String,
    /// The name of the subcategory (e.g.: clothing)
    pub subcategory: String,
    /// The name of the author who created the document
    pub author: String,
    /// The indicator that represents if the document is waiting to be processed (processed = true, needs to be processed = false)
    pub process_ind: bool,
    /// The Unix Epoch time when the document was last updated, (e.g.: 1555972752)
    pub last_updated: u64,
    /// The JSON value that represents the data from the data source managed by the DaaS document
    pub data_obj: Value,
}

/// Represents an new DaaS document (before it has been saved and assigned a _rev value)
#[derive(Serialize, Deserialize, Debug, Clone)]
struct DaaSDocNoRev{
    /// The unique identifier
    pub _id: String,
    /// The name of the data source
    pub source_name: String,
    /// The unique identifier that the data source provides
    pub source_uid: usize,
    /// The name of the category (e.g.: order)
    pub category: String,
    /// The name of the subcategory (e.g.: clothing)
    pub subcategory: String,
    /// The name of the author who created the document
    pub author: String,
    /// The indicator that represents if the document is waiting to be processed (processed = true, needs to be processed = false)
    pub process_ind: bool,
    /// The Unix Epoch time when the document was last updated, (e.g.: 1555972752)
    pub last_updated: u64,
    /// The JSON value that represents the data from the data source managed by the DaaS document
    pub data_obj: Value,
}
```

To give these objects functionality, we use the `impl` syntax on the object, (e.g.: `DaaSDoc`). To make it an Object Oriented implementation of the structure, we define the `new()` function as the constructor.

```
impl DaaSDoc {
    pub fn new(src_name: String, src_uid: usize, cat: String, subcat: String, auth: String, data: Value) -> DaaSDoc {
        DaaSDoc{
            _id: make_id(cat.clone(), subcat.clone(), src_name.clone(), src_uid),
            _rev: None,
            source_name: src_name,
            source_uid: src_uid,
            category: cat,
            subcategory: subcat,
            author: auth,
            process_ind: false,
            last_updated: get_unix_now(),
            data_obj: data,
        }
    }
}
```

We continue to add functionality by constructing the other necessary functions in the DaaSDoc implementaiton.

```
    pub fn data_obj_as_ref(&mut self) -> &mut Value {
        &mut self.data_obj
    } 

    pub fn from_serialized(serialized: &str) -> DaaSDoc {
		serde_json::from_str(&serialized).unwrap()
    }

    pub fn serialize(&mut self) -> String {
		serde_json::to_string(&self).unwrap()
    }

    pub fn serialize_without_rev(&mut self) -> String {
        let no_rev: DaaSDocNoRev = DaaSDocNoRev {
            _id: self._id.clone(),
            source_name: self.source_name.clone(),
            source_uid: self.source_uid.clone(),
            category: self.category.clone(),
            subcategory: self.subcategory.clone(),
            author: self.author.clone(),
            process_ind: self.process_ind.clone(),
            last_updated: get_unix_now(),
            data_obj: self.data_obj.clone(),
        };

        let serialized: String = serde_json::to_string(&no_rev).unwrap();

        serialized
    }    
```

> Make sure to rerun the `cargo test` command to ensure all your tests pass.