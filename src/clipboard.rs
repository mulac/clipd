use toml;
use serde_derive::{Serialize, Deserialize};
use std::io::Read;

trait Clipboard {
    fn add(&mut self, key: String, value: String);
    fn get(&self, key: String) -> String;
    fn show();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ClipdFS {
    root_path: String,
    count: i32,
    custom_keys: std::collections::HashMap<String, i32>,
}

impl ClipdFS {
    pub fn new() -> ClipdFS {
        ClipdFS::from_file("~/.clipd/default/config.toml")
    }

    fn from_file(file: &str) -> ClipdFS {
        let mut toml_str = String::new();
        std::fs::File::open(file).and_then(|mut f| f.read_to_string(&mut toml_str)).unwrap();
        toml::from_str(&toml_str).unwrap() // TODO: use `unwrap_or_else` to return create a directory and config if not exists
    }

    fn save(&self) -> std::io::Result<()> {
        let toml_str = toml::to_string(&self).unwrap();
        std::fs::write(self.root_path.as_str(), toml_str)?;
        Ok(())
    }
}

impl Clipboard for ClipdFS {
    fn add(&mut self, key: String, value: String) {
        /* 
        /    Increment the item count
        /    If needed, add mapping between custom key and conceptual number key
        /    Calculate actual number key
        /  Create file /clipd/default/[ACTUAL_NUMBER_KEY]
        /    Copy value into file
        */
        self.count += 1;
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
        key
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
        let toml_struct = super::ClipdFS {
            root_path: "/tmp/clipd/config.toml".to_string(),
            count: 0,
            custom_keys: std::collections::HashMap::new(),
        };

        // serialize ting
        let toml_serialized = toml::to_string(&toml_struct).unwrap();
        
        // write to file
        std::fs::write("/tmp/clipd/config.toml", toml_serialized.clone()).unwrap();
        // read from file
        let mut toml_str = String::new();
        std::fs::File::open("/tmp/clipd/config.toml").and_then(|mut f| f.read_to_string(&mut toml_str)).unwrap();

        // deserialize ting
        let toml_deserialized: super::ClipdFS = toml::from_str(&toml_str).unwrap();

        assert_eq!(toml_serialized, toml_str);
        assert_eq!(toml_deserialized, toml_struct);
    }
}