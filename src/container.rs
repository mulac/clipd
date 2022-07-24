pub trait Container {
    /// Add a new value to the container
    ///
    /// `key` is an optional String that can be associated to the value
    /// and later used with `Container.get`
    fn add(&mut self, key: Option<String>, value: String);

    /// Returns Some(value) if the given `key` is found, else None
    ///
    /// `key` can either be a custom_key given when the value was added with
    /// `Container.add` or it can be a zero indexed number that will be associated
    /// with the nth last added value.
    fn get(&self, key: Option<String>) -> Option<String>;

    /// Returns a pretty-formatted string of values stored in the Container.
    /// It is not guaranteed to show all values and should only be used for human
    /// display
    fn show(&self, n: usize) -> String;

    /// **WARNING** This will completely and permanently empty the container of all values
    fn clear(&mut self);
}

pub enum ContainerType {
    ClipdFs,
}

pub fn create(ctype: ContainerType, container: String) -> impl Container {
    match ctype {
        ContainerType::ClipdFs => clipd_fs::open(container),
    }
}

/// A filesystem oriented implementation of a Container
/// ---
mod clipd_fs {
    use serde_derive::{Deserialize, Serialize};
    use std::{io::Read, path::PathBuf};
    use tabled::{builder::Builder, Style};
    use uuid::Uuid;

    use crate::util::truncate_utf8;

    static ROOT: &'static str = ".clipd";
    static CONFIG_NAME: &'static str = "config.toml";
    fn path(name: &str) -> PathBuf {
        std::env::home_dir().unwrap().join(ROOT).join(name)
    }
    fn config_path(name: &str) -> PathBuf {
        std::env::home_dir()
            .unwrap()
            .join(ROOT)
            .join(name)
            .join(CONFIG_NAME)
    }

    pub fn open(name: String) -> impl super::Container {
        let path = path(&name);
        let config_path = config_path(&name);

        if path.exists() {
            return Container::from_file(config_path);
        }

        std::fs::create_dir_all(&path)
            .unwrap_or_else(|err| panic!("failed to create dirs at {path:?}: {err}"));
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
        custom_keys: Vec<String>,
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
            std::fs::File::open(&config_path)
                .and_then(|mut f| f.read_to_string(&mut toml_str))
                .unwrap();
            // TODO: handle out of sync config files more gracefully
            toml::from_str(&toml_str)
            .unwrap_or_else(|err| panic!("[Container] Failed to deserialise config file at {config_path:?},
            it is possible config file has been edited or become out of sync with the in memory representation of a Container: {err}"))
        }

        fn save(&self) {
            let toml_str = toml::to_string(&self).unwrap_or_else(|err| {
                panic!("[Container] Failed to serialise container {self:?}: {err}")
            });
            std::fs::write(self.config_path(), toml_str).unwrap_or_else(|err| {
                panic!(
                    "[Container] Failed to save container {} at path {:?}: {}",
                    self.name,
                    self.config_path(),
                    err
                )
            });
        }

        fn get_value(&self, uuid: &String) -> std::io::Result<String> {
            std::fs::read_to_string(self.path().join(uuid))
        }

        fn path(&self) -> PathBuf {
            path(&self.name)
        }

        fn config_path(&self) -> PathBuf {
            config_path(&self.name)
        }
    }

    impl super::Container for Container {
        fn add(&mut self, key: Option<String>, value: String) {
            // TODO: If key is already in use, delete old reference to key in self.items
            let item = Item::new(key.map(|k| vec![k]));

            self.count += 1;
            for k in item.custom_keys.as_slice() {
                self.custom_keys.insert(k.clone(), item.uuid.clone());
            }

            self.ordered_items.push_front(item);

            std::fs::write(
                self.path().join(&self.ordered_items.front().unwrap().uuid),
                value,
            )
            .unwrap();
            self.save();
        }

        fn get(&self, key: Option<String>) -> Option<String> {
            let key = key.unwrap_or("0".to_string());
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

        fn show(&self, n: usize) -> String {
            let val_limit = 32;
            let mut view = Vec::new();

            for (i, item) in self.ordered_items.iter().enumerate() {
                if i == n {
                    break;
                }
                let val = self.get_value(&item.uuid).unwrap();
                view.push([
                    // ID
                    item.uuid.clone().get(..8).unwrap().to_string(),
                    // Custom Keys
                    format!("{:?}", item.custom_keys),
                    // Value Preview
                    if val.len() <= val_limit {
                        val
                    } else {
                        truncate_utf8(val.as_str(), val_limit).to_string() + "..."
                    },
                ]);
            }

            Builder::from_iter(view)
                .set_columns(["ID", "Custom Keys", "Value"])
                .index()
                .build()
                .with(Style::rounded())
                .to_string()
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
            std::fs::write(
                "/home/calum/.clipd/default/config.toml",
                toml_serialized.clone(),
            )
            .unwrap();
            // read from file
            let mut toml_str = String::new();
            std::fs::File::open("/home/calum/.clipd/default/config.toml")
                .and_then(|mut f| f.read_to_string(&mut toml_str))
                .unwrap();

            // deserialize ting
            let toml_deserialized: super::Container = toml::from_str(&toml_str).unwrap();

            assert_eq!(toml_serialized, toml_str);
            assert_eq!(toml_deserialized, toml_struct);
        }
    }
}
