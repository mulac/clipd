pub trait Clipboard {
    fn add(&mut self, key: Option<String>, value: String);
    fn get(&self, key: String) -> String;
    fn show();
}

pub mod clipd_fs {
    
use serde_derive::{Serialize, Deserialize};
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

static ROOT_PATH: &'static str = "/home/calum/.clipd";
static CONFIG_FNAME: &'static str = "config.toml";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    name: String,
    count: u32,
    ordered_uuids: std::vec::Vec<String>,
    custom_keys: std::collections::HashMap<String, String>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            name: "default".to_string(),
            count: 0,
            ordered_uuids: vec![],
            custom_keys: std::collections::HashMap::new(),
        }
    }

    pub fn new_with_name(name: String) -> Container {
        Container {
            name: name,
            count: 0,
            ordered_uuids: vec![],
            custom_keys: std::collections::HashMap::new(),
        }
    }

    pub fn from_file(path: &str) -> Container {
        let mut toml_str = String::new();
        std::fs::File::open(path).and_then(|mut f| f.read_to_string(&mut toml_str)).unwrap();
        toml::from_str(&toml_str).unwrap() // TODO: use `unwrap_or_else` to return create a directory and config if not exists
    }

    fn save(&self) -> std::io::Result<()> {
        let toml_str = toml::to_string(&self).unwrap();
        std::fs::write(self.config_path(), toml_str)?;
        Ok(())
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(ROOT_PATH).join(&self.name)
    }

    fn config_path(&self) -> PathBuf {
        self.path().join(CONFIG_FNAME)
    }
}

impl super::Clipboard for Container {
    fn add(&mut self, key: Option<String>, value: String) {
        /* 
        /    Increment the item count
        /    If needed, add mapping between custom key and conceptual number key
        /    Calculate actual number key
        /  Create file /clipd/default/[ACTUAL_NUMBER_KEY]
        /    Copy value into file
        */

        let uuid = Uuid::new_v4();

        self.count += 1;
        self.ordered_uuids.push(uuid.to_string());
        if let Some(k) = key {
            self.custom_keys.insert(k, uuid.to_string());
        }

        std::fs::write(self.path().join(uuid.to_string()), value).unwrap();
        self.save().unwrap();
    }

    fn get(&self, key: String) -> String {
        /* 
        /    Extract item count
        /    If needed, get associated conceptual number key from custom key
        /    calculate actual number key
        /  Open ~/clipd/default/[ACTUAL_NUMBER_KEY].txt
        /    Read entire file to get the value
        /  Return value
        */

        let mut path = self.path();

        // TODO: move to a fancy match
        if let Some(uuid) = self.custom_keys.get(&key) {
            path.push(uuid);
        } else if let Ok(n) = key.parse::<u32>() {
            let idx = self.ordered_uuids.len().checked_sub((n+1) as usize).unwrap();
            path.push(&self.ordered_uuids[idx])
        } else {
            return "".to_string();
        }
        std::fs::read_to_string(path).unwrap()
    }

    fn show() {

    }
}


#[cfg(test)]
mod test {
    use super::Read;

    #[test]
    fn test_clipdfs_roundtrip() {
        // Create ting
        let toml_struct = super::Container::new();
        println!("{:?}", toml_struct);
        // serialize ting
        let toml_serialized = toml::to_string(&toml_struct).unwrap();
        
        // write to file
        std::fs::write("/home/calum/.clipd/default/config.toml", toml_serialized.clone()).unwrap();
        // read from file
        let mut toml_str = String::new();
        std::fs::File::open("/home/calum/.clipd/default/config.toml").and_then(|mut f| f.read_to_string(&mut toml_str)).unwrap();

        // deserialize ting
        let toml_deserialized: super::Container = toml::from_str(&toml_str).unwrap();

        assert_eq!(toml_serialized, toml_str);
        assert_eq!(toml_deserialized, toml_struct);
    }
}
}