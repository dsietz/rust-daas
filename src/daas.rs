//! The `daas` module provides functionality and Object Oriented implementation of a Data as a Service (DaaS) object.
//!
//! # Examples
//!
//!
//! Create DaaS object that represents a new purchase of clothing from an online store.
//!
//! ```
//! #[macro_use] 
//! extern crate serde_json;
//! extern crate daas;
//!
//! use serde_json::value::*;
//! use daas::daas::DaaSDoc;
//!
//! fn main() {
//!		let src = "iStore".to_string();
//!		let uid = 5000;
//!		let cat = "order".to_string();
//!		let sub = "clothing".to_string();
//!		let auth = "istore_app".to_string();
//!		let data = json!({
//!         "product": "leather coat",
//!         "quantity": 1,
//!		    "status": "new"
//!		});
//! 
//!		let doc = DaaSDoc::new(src, uid, cat, sub, auth, data);
//! 
//!     assert_eq!(doc.source_uid, uid);
//! }
//! ```

use super::*;
use serde_json::value::*;

/// Delimiter used for building the unique identifier value for the DaaS document
static DELIMITER: &str = "|";

/// A shared function that returns the unique identifier
///
/// # Arguments
///
/// * `cat: String - The name of the category (e.g.: order).</br>
/// * `sub: String - The name of the subcategory (e.g.: clothing).</br>
/// * `src_name: String - The name of the data source.</br>
/// * `src_uid: usize - The unique identifier that the data source provided.</br>
/// 
/// #Example
///
/// ```
/// extern crate daas;
///
/// use daas::daas::make_id;
///
/// fn main() {
///		let id = make_id("order".to_string(),"clothing".to_string(),"iStore".to_string(),111000);
///		println!("{:?}", id);
/// }
/// ```
pub fn make_id(cat: String, subcat: String, src_name: String, src_uid: usize) -> String {
    format!("{}{}{}{}{}{}{}",cat, DELIMITER, subcat, DELIMITER, src_name, DELIMITER, src_uid).to_string()
}    

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


impl DaaSDoc {
    /// Constructor
    /// 
    /// # Arguments
    /// 
    /// * `src_name: String - The name of the data source.</br>
    /// * `src_uid: usize - The unique identifier that the data source provided.</br>
    /// * `cat: String - The name of the category (e.g.: order).</br>
    /// * `sub: String - The name of the subcategory (e.g.: clothing).</br>
    /// * `auth: String - The name of the auithor who created the document.</br>
    /// * `data: Value - The json value that represents the data from the data source managed by the DaaS document.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// #[macro_use] 
    /// extern crate serde_json;
    /// extern crate daas;
    ///
    /// use serde_json::value::*;
    /// use daas::daas::DaaSDoc;
    ///
    /// fn main() {
    ///     let src = "iStore".to_string();
    ///     let src = "iStore".to_string();
    ///     let uid = 5000;
    ///     let cat = "order".to_string();
    ///     let sub = "clothing".to_string();
    ///     let auth = "istore_app".to_string();
    ///     let data = json!({
    ///         "status": "new"
    ///     });
    ///     
    ///     let doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
    ///     
    ///     println!("{:?}", doc._id);
    /// }
    /// ```
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

    /// Returns the data source json value as a reference
    /// 
    /// #Example
    ///
    /// ```
    /// #[macro_use] 
    /// extern crate serde_json;
    /// extern crate daas;
    ///
    /// use serde_json::value::*;
    /// use daas::daas::DaaSDoc;
    ///
    /// fn main() {
    ///     let src = "iStore".to_string();
    ///     let src = "iStore".to_string();
    ///     let uid = 5000;
    ///     let cat = "order".to_string();
    ///     let sub = "clothing".to_string();
    ///     let auth = "istore_app".to_string();
    ///     let data = json!({
    ///         "status": "new"
    ///     });
    ///     
    ///     let doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
    ///     
    ///     assert_eq!(doc.data_obj.get("status").unwrap(), "new");
    /// }
    /// ```
    pub fn data_obj_as_ref(&mut self) -> &mut Value {
        &mut self.data_obj
    }    

    /// Constructs a DaaSDoc object from a serialized string
    /// 
    /// # Arguments
    /// 
    /// * `serialized: &str - The string that represents the serialized object.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate daas;
    ///
    /// use daas::daas::DaaSDoc;
    ///
    /// fn main() {
    ///     let serialized = r#"{"_id":"order|clothing|iStore|5000","_rev":null,"source_name":"iStore","source_uid":5000,"category":"order","subcategory":"clothing","author":"istore_app","process_ind":false,"last_updated":1553988607,"data_obj":{"status":"new"}}"#;
    ///     let doc = DaaSDoc::from_serialized(&serialized);
  	///     
    ///     assert_eq!(doc.source_uid, 5000);
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> DaaSDoc {
		serde_json::from_str(&serialized).unwrap()
    }

    /// Serializes the DaaSDoc object
    /// 
    /// #Example
    ///
    /// ```
    /// #[macro_use] 
    /// extern crate serde_json;
    /// extern crate daas;
    ///
    /// use serde_json::value::*;
    /// use daas::daas::DaaSDoc;
    ///
    /// fn main() {
    ///     let src = "iStore".to_string();
    ///     let src = "iStore".to_string();
    ///     let uid = 5000;
    ///     let cat = "order".to_string();
    ///     let sub = "clothing".to_string();
    ///     let auth = "istore_app".to_string();
    ///     let data = json!({
    ///         "status": "new"
    ///     });
    ///     
    ///     let mut doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
    ///     
    ///     println!("{:?}", doc.serialize());
    /// }
    /// ```
    pub fn serialize(&mut self) -> String {
		serde_json::to_string(&self).unwrap()
    }

    /// Serializes the DaaSDoc object without the _rev attribute
    /// 
    /// #Example
    ///
    /// ```
    /// #[macro_use] 
    /// extern crate serde_json;
    /// extern crate daas;
    ///
    /// use serde_json::value::*;
    /// use daas::daas::DaaSDoc;
    ///
    /// fn main() {
    ///     let src = "iStore".to_string();
    ///     let src = "iStore".to_string();
    ///     let uid = 5000;
    ///     let cat = "order".to_string();
    ///     let sub = "clothing".to_string();
    ///     let auth = "istore_app".to_string();
    ///     let data = json!({
    ///         "status": "new"
    ///     });
    ///     
    ///     let mut doc = DaaSDoc::new(src.clone(), uid, cat.clone(), sub.clone(), auth.clone(), data);
    ///     
    ///     println!("{:?}", doc.serialize());
    /// }
    /// ```
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
}

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