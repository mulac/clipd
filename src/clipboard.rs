pub trait Clipboard {
    fn add(&mut self, key: Option<String>, value: String);
    fn get(&self, key: String) -> Option<String>;
    fn show(&self) -> String;
    fn clear(&mut self);
}

pub enum ClipboardType {
    ClipdFs
}

pub fn create(ctype: ClipboardType, container: String) -> impl Clipboard {
    match ctype {
        ClipboardType::ClipdFs => {
            clipd_fs::open(container)
        }
    }
}

mod clipd_fs {
    
use serde_derive::{Serialize, Deserialize};
use std::{io::Read, path::PathBuf};
use uuid::Uuid;
use tabled::{builder::Builder, Style};

use super::Clipboard;

static ROOT_PATH: &'static str = "/home/calum/.clipd";
static CONFIG_FNAME: &'static str = "config.toml";
fn path(name: &str)-> PathBuf{PathBuf::from(ROOT_PATH).join(name)}
fn config_path(name: &str)-> PathBuf{path(name).join(CONFIG_FNAME)}

pub fn open(name: String) -> impl Clipboard {
    let path = path(&name);
    let config_path = config_path(&name);

    if path.exists() {
        return Container::from_file(config_path)
    }

    std::fs::create_dir_all(path).expect("failed to create dirs at {path}");
    let c = Container::new(name);
    c.save();
    c
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Container {
    name: String,
    count: u32,
    ordered_items: std::collections::VecDeque<Item>,
    custom_keys: std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Item {
    uuid: String,
    custom_keys: Vec<String>
}

impl Item {
    fn new(key: Option<Vec<String>>) -> Item {
        Item {
            uuid: Uuid::new_v4().to_string(),
            custom_keys: key.unwrap_or_default(),
        }
    }
}

impl Container {
    fn new(name: String) -> Container {
        Container {
            name: name,
            count: 0,
            ordered_items: std::collections::VecDeque::new(),
            custom_keys: std::collections::HashMap::new(),
        }
    }

    fn from_file(config_path: PathBuf) -> Container {
        let mut toml_str = String::new();
        std::fs::File::open(config_path).and_then(|mut f| f.read_to_string(&mut toml_str)).unwrap();
        toml::from_str(&toml_str).unwrap() // TODO: use `unwrap_or_else` to return create a directory and config if not exists
    }

    fn save(&self) {
        let toml_str = toml::to_string(&self).unwrap();
        std::fs::write(self.config_path(), toml_str).expect("can't create new container {name} at path {c.config_path()}");
    }

    fn get_value(&self, uuid: &String) -> std::io::Result<String> {
        std::fs::read_to_string(self.path().join(uuid))
    }

    fn path(&self) -> PathBuf {path(&self.name)}

    fn config_path(&self) -> PathBuf {config_path(&self.name)}
}

impl super::Clipboard for Container {
    fn add(&mut self, key: Option<String>, value: String) {
        // TODO: If key is already in use, delete old reference to key in self.items 

        let item = Item::new(key.map(|k| vec![k]));

        self.count += 1;
        for k in item.custom_keys.as_slice() {
            self.custom_keys.insert(k.clone(), item.uuid.clone());
        }

        self.ordered_items.push_front(item);

        std::fs::write(self.path().join(&self.ordered_items.front().unwrap().uuid), value).unwrap();
        self.save();
    }

    fn get(&self, key: String) -> Option<String> {
        if let Ok(n) = key.parse::<usize>() {
            // TODO: out of bounds check
            return std::fs::read_to_string(self.path().join(&self.ordered_items[n].uuid)).ok();
        } 

        let uuid = self.custom_keys.get(&key)?;
        std::fs::read_to_string(self.path().join(uuid)).ok()
    }

    fn clear(&mut self) {
        std::fs::remove_dir_all(self.path()).unwrap();
    }

    fn show(&self) -> String {
        let mut view = Vec::new();
        for item in &self.ordered_items {
            view.push(
                [item.uuid.clone(), format!("{:?}", item.custom_keys), self.get_value(&item.uuid).unwrap()]
            );
        }

        Builder::from_iter(view).index().build().with(Style::rounded()).to_string()
    }
}


#[cfg(test)]
mod test {
    use super::Read;

    #[test]
    fn test_clipdfs_roundtrip() {
        // Create ting
        let toml_struct = super::Container::new("default".to_string());
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