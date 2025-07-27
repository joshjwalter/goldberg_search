// build a target profile that can be saved to something like a json file (idk prob not a custom file type cuz that would suck) and be able to
// resume editing and build a case on somebody with customn data gathered without automation too (eventually maybe add webscraping or find ways to automate everyting)
use serde_json::Value;

pub struct Target {
    // TODO: add a field that keeps each of the entried as one Vec Each, or maybe just make the Target have identifiers or something because...
    // just the index might not be enogh
    pub first_names_list: Vec<String>,
    pub last_names_list: Vec<String>,
    pub address_list: Vec<String>,
}

// TODO: add a trait that saves to heroku sql database or something
// maybe just json file

pub trait AddParcel {
    fn add_parcel(self, parcel: &Value) -> Target;
}

impl AddParcel for Target {
    fn add_parcel(mut self, parcel: &Value) -> Target {
        let name: String = parcel["attributes"]["MAILING_NAME"].to_string();
        let name_seperated: Vec<&str> = name.trim().split(" ").collect();

        // names in the DB are usually Last name, then First name
        self.first_names_list.push(name_seperated[1].to_string());
        self.last_names_list.push(name_seperated[0].to_string());

        self.address_list.push(parcel["attributes"]["MAILING_ADDRESS"].to_string());

        return self
    }
}