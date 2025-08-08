mod target;
use target::{Target};


mod state;
use reqwest::blocking::Client;
use serde_json::Value;
use std:: io;
use std::io::Write;


fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().expect("Flush Failed");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("failed to read line");
    return user_input;
}

fn display_parcels(parcel_vec: &Vec<Value>) {
    let mut number: u32 = 1;
    for x in parcel_vec {
        println!("({})", number);
        println!("\n-----START-----");
        println!("Mailing Name: {}", x["attributes"]["MAILING_NAME"]);
        println!("Land Use Description: {}", x["attributes"]["LAND_USE_DESCRIPTION"]);
        println!("Alltoment: {}", x["attributes"]["ALLOTMENT"]);
        println!("Mailing Address: {}", x["attributes"]["MAILING_ADDRESS"]);
        println!("Billing Address: {}", x["attributes"]["BILLING_ADDRESS1"]);
        println!("Appraised Building Value: {}", x["attributes"]["APPRAISED_BUILDING_VALUE"]);
        println!("Appraised Land Value: {}", x["attributes"]["APPRAISED_LAND_VALUE"]);
        println!("Appraised Total Value: {}", x["attributes"]["APPRAISED_TOTAL_VALUE"]);
        println!("------END------");
        number += 1;
    }
}

// Depricated, used to find the db size but unneeded unless messing with upper bounds and db size changes
fn db_size() -> u32 {
    const OFFSET: u32 = 2000;
    let mut iterations: u32 = 0;
    let mut entries: u32 = 0;
    let mut full_size: bool = true;
    let client = Client::new();

    while full_size {
        let request_string: String = format!("https://scgisa.starkcountyohio.gov/arcgis/rest/services/Auditor/StarkCountyParcels_Viewer/MapServer/0/query?f=json&where=1%3D1&returnGeometry=false&outFields=*&resultRecordCount=2000&resultOFFSET={}", (iterations*OFFSET).to_string());
        //println!("Requesting records from {} to {}", ((iterations*OFFSET)+1).to_string(), ((iterations+1)*OFFSET).to_string());
        let result: Value = client.get(request_string).send().unwrap().json().unwrap();
        let length = result["features"].as_array().unwrap().len() as u32;
        if length < entries {
            entries = length;
            full_size = false;
        } else {
            entries = length;
            iterations += 1;
        }
    }
    //println!("Total number of records: {}", ((iterations*OFFSET)+entries).to_string());
    return (iterations*OFFSET)+entries;
}

fn main() {
    println!("Welcome to Goldberg Search");
    let choice = input("Choose a procedure: \n1. Input a profile \n2. Search Stark Database\n\nChoice: ");
    match choice.trim() {
        "1" => {println!("Inputting a profile is not yet supported")},
        "2" => {stark_db_search()},
        _ => {
            println!("Please choose a real option");
            //main();
            //maybe add a while loop to go until a choice is chosen? or create a function with a while loop so there isnt a while loop in main
        }
    }    
}