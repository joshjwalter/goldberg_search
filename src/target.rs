// build a target profile that can be saved to something like a json file (idk prob not a custom file type cuz that would suck) and be able to
// resume editing and build a case on somebody with customn data gathered without automation too (eventually maybe add webscraping or find ways to automate everyting)
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Parcel {
    first_name: String,
    last_name: String,
    address: String
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Target {
    // TODO: add a field that keeps each of the entried as one Vec Each, or maybe just make the Target have identifiers or something because...
    // just the index might not be enogh
    pub parcel_list: HashMap<String, Parcel>,
}

// TODO: add a trait that saves to heroku sql database or something
// maybe just json file


//change form returning Target to returning () so that it just edits the borrowed self instead of a new Target

impl Target {
    pub fn new() -> Self {
        Self {
            parcel_list: HashMap::new()
        }
    }
    pub fn add_parcel(&mut self, parcel: &Value) {
        let current_parcel_list_length: usize = self.parcel_list.len();
        let name: String = parcel["attributes"]["MAILING_NAME"].to_string();
        let name_seperated: Vec<&str> = name.trim().split(" ").collect();

        let addr = parcel["attributes"]["MAILING_ADDRESS"]
            .as_str()
            .unwrap_or("")
            .to_string(); // safe extract

        // names in the DB are usually Last name, then First name
        match name_seperated.len() {
            0 => {
                self.parcel_list
                    .insert(
                        current_parcel_list_length.to_string(),
                        Parcel {
                            first_name: String::new(), 
                            last_name: String::new(), 
                            address: String::from(addr)
                        }
                    );
            },
            1 => {
                self.parcel_list
                    .insert(
                        current_parcel_list_length.to_string(),
                        Parcel {
                            first_name: String::new(), 
                            last_name: String::from(name_seperated[0]), 
                            address: String::from(addr)
                        }
                    );
            },
            _ => {
                self.parcel_list
                    .insert(
                        current_parcel_list_length.to_string(),
                        Parcel {
                            first_name: String::from(name_seperated[1]), 
                            last_name: String::from(name_seperated[0]), 
                            address: String::from(addr)
                        }
                    );
            }
        }
    }
    pub fn save_to_json(&self, file_path: &str) -> Result<(), std::io::Error> {
        let json_string = serde_json::to_string_pretty(self).unwrap();

        let mut file = File::create(file_path)?;

        file.write_all(json_string.as_bytes())?;

        println!("Profile exported to {}", file_path);
        Ok(())
    }
}

//IMPL PARCEL::NEW