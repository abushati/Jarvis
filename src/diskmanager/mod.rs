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
        println!("{}",s);
        connection.execute(
            s,
        ).unwrap();
        println!("{:?}",self)
    }
    pub fn get_key_meta(self) {
        let s = format!("Select * from metadata where id = '{}' ",self.public_file_path);
        let connection = sqlite::open("jarvis.db").unwrap();
        // let stmt = connection.prepare(s).unwrap();
        for row in connection
            .prepare(s.clone())
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap()){
                let e: i64 = row.read("json_data");
                print!("{}",&e)
        }
        // println!("{:?}",res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ManagerActionsEntry {
    pub actionType: String,
    pub file_bytes: Vec<u8>, 
    pub fileData: Option<FileUploadData>
}