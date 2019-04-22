use super::*;
use serde_json::value::*;

static DELIMITER: &str = "|";

pub fn make_id(cat: String, subcat: String, src_name: String, src_uid: usize) -> String {
    format!("{}{}{}{}{}{}{}",cat, DELIMITER, subcat, DELIMITER, src_name, DELIMITER, src_uid).to_string()
}    

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DaaSDoc{
    pub _id: String,
    pub _rev: Option<String>,
    pub source_name: String,
    pub source_uid: usize,
    pub category: String,
    pub subcategory: String,
    pub author: String,
    pub process_ind: bool,
    pub last_updated: u64,
    pub data_obj: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DaaSDocNoRev{
    pub _id: String,
    pub source_name: String,
    pub source_uid: usize,
    pub category: String,
    pub subcategory: String,
    pub author: String,
    pub process_ind: bool,
    pub last_updated: u64,
    pub data_obj: Value,
}

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