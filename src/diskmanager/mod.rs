use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub file_id: String,
    pub file_key: String,
    pub insert_time: String,
    // file_type: String
}
impl MetaData {
    pub fn save(self) {
        let connection = sqlite::open("jarvis.db").unwrap();
        connection.execute(
            "CREATE TABLE IF NOT EXISTS metadata (id String PRIMARY KEY, json_data TEXT NOT NULL)",
        ).unwrap();
        let json = serde_json::to_string(&self).unwrap();
        let s = format!("INSERT INTO metadata (id, json_data) VALUES ('{}','{}')",self.file_key,json);
        println!("{}",s);
        connection.execute(
            s,
        ).unwrap();
        println!("{:?}",self)
    }
    pub fn get_key_meta(self, id: String) {
        let s = format!("Select * from metadata where id = '{}' ",self.file_key);
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