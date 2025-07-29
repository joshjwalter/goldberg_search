// build a target profile that can be saved to something like a json file (idk prob not a custom file type cuz that would suck) and be able to
// resume editing and build a case on somebody with customn data gathered without automation too (eventually maybe add webscraping or find ways to automate everyting)
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Target {
    // TODO: add a field that keeps each of the entried as one Vec Each, or maybe just make the Target have identifiers or something because...
    // just the index might not be enogh
    pub first_names_list: Vec<String>,
    pub last_names_list: Vec<String>,
    pub address_list: Vec<String>,
}

// TODO: add a trait that saves to heroku sql database or something
// maybe just json file

pub trait Expandable {
    fn add_parcel(self, parcel: &Value) -> Target;
}

impl Expandable for Target {
    fn add_parcel(mut self, parcel: &Value) -> Target {
        let name: String = parcel["attributes"]["MAILING_NAME"].to_string();
        let name_seperated: Vec<&str> = name.trim().split(" ").collect();

        // names in the DB are usually Last name, then First name
        if name_seperated.len() >= 2 {
            self.first_names_list.push(name_seperated[1].to_string());
            self.last_names_list.push(name_seperated[0].to_string());
        } else {
            self.last_names_list.push(name.to_string());
        }


        self.address_list
            .push(parcel["attributes"]["MAILING_ADDRESS"].to_string());

        self
    }
}

pub trait Saveable {
    /// Saves the struct as a JSON file.
    fn save_to_json(&self, file_path: &str) -> Result<(), std::io::Error>;
}

impl Saveable for Target {
    fn save_to_json(&self, file_path: &str) -> Result<(), std::io::Error> {
        let json_string = serde_json::to_string_pretty(self).unwrap();

        let mut file = File::create(Path::new(file_path))?;

        file.write_all(json_string.as_bytes())?;

        println!("Profile exported to {}", file_path);
        Ok(())
    }
}

pub trait Importable {
    /// Imports JSON file to Target struct
    fn import_from_json(&self, file_path: &str)-> Target;
}

impl Importable for Target {
    fn import_from_json(&self, file_path: &str)-> Target {
        let file = File::open(Path::new(file_path)).unwrap();
        
        let imported_target: Target = serde_json::from_reader(file).unwrap();

        return imported_target;
    }
}