use serde::{Serialize, Deserialize};
use crate::syner::FileUploadData;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub file_name: String,
    pub public_file_path: String,
    pub _internal_file_id: String,
    pub _internal_file_path: String,
    pub insert_time: String,
    pub update_time:  String,
    pub user: Option<u8>
    // file_type: String
}
impl MetaData {
    pub fn save(&mut self) {
        let connection = sqlite::open("jarvis.db").unwrap();
        connection.execute(
            "CREATE TABLE IF NOT EXISTS metadata (id String PRIMARY KEY, json_data TEXT NOT NULL)",
        ).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        let s = format!("INSERT INTO metadata (id, json_data) VALUES ('{}','{}')",self.public_file_path, json);
        connection.execute(
            s,
        ).unwrap();
    }

    pub fn delete(self) {
        let s = format!("delete from metadata where id = '{}' ",self.public_file_path);
        let connection = sqlite::open("jarvis.db").unwrap();
        connection.execute(s);
    }

    pub fn get_key_meta(public_file_path: String) -> Result<MetaData,String> {
        let s = format!("Select * from metadata where id = '{}' ",public_file_path);
        let connection = sqlite::open("jarvis.db").unwrap();
        // let stmt = connection.prepare(s).unwrap();
        for row in connection 
            .prepare(s.clone())
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()){
                let e: &str = row.read("json_data");
                let metadata:MetaData = serde_json::from_str(e).unwrap();
                return Ok(metadata)
        }


        return Err("bad".to_string());        
    }
}


//Todo: need to check why some of these fields are options
#[derive(Serialize, Deserialize)]
pub struct ManagerActionsEntry {
    pub action_type: ManagerAction,
    pub file_bytes: Option<Vec<u8>>, 
    pub fileData: Option<FileUploadData>,
    pub file_pub_key: Option<String>,
    pub basket_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ManagerAction {
    WriteFile,
    ReadFile,
    UpdateFile,
    DeleteFile,
    CreateBasket,
    DeleteBasket,
    EditBaskets
}


