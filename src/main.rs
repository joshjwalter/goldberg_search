mod target;
use target::{Target, Parcel};
mod state;
use state::{State};

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

fn display_parcels(parcel_vec: &Vec<Parcel>) {
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

    let web_client = Client::new();

    let mut app_state: State = State::new();
    
    //make this a function for creating a new Target search and create a framework where I can initialize one of these or browse other Targets or export or whatnot
    let new_target_identifier = input("What do you want to call the Target you are pursuing? ");
    app_state.add_target(Target::new(), new_target_identifier.clone());
    app_state.selected_target_identifier = new_target_identifier;

    let name = input("Input Target Name (Last First): ").to_lowercase();
    // switch to loop that auto displays 5 each page
    let result_record_count = input("How many max results would you like per page? (up to 2000)");
    let mut viewing_pages: bool = true;
    let mut page: u32 = 0;

    while viewing_pages {
        let request_string: String = format!("https://scgisa.starkcountyohio.gov/arcgis/rest/services/Auditor/StarkCountyParcels_Viewer/MapServer/0/query?f=json&where=LOWER%28OWNER%29%20LIKE%20%27%25{}%25%27&returnGeometry=false&outFields=*&resultRecordCount={}&resultOFFSET={}", name, result_record_count, page);
        let  result: Value = web_client.get(request_string).send().unwrap().json().unwrap();
        let result_vec_values = result["features"].as_array().unwrap();
        let result_vec_parcels: Vec<Parcel>;
        for x in result_vec_values {
            result_vec_parcels.push(Parcel::new(x["attributes"]["MAILING_NAME"][1].to_string(), x["attributes"]["MAILING_NAME"][0].to_string(), x["attributes"]["MAILING_ADDRESS"].to_string()))
        }
        if result_vec_parcels.len() == 0 {
            println!("No more results, restarting from the beginning");
            page = 0;
            continue;
        } else {
            display_parcels(result_vec);
            let input_string = input("Select all choices that seem reasonable (0 for next page) ('end' to end search) (split by comma)");
            let choice = input_string.trim().split(",").collect::<Vec<&str>>();
            if choice.clone()[0] == "0" {
                page += 1;
                continue;
            } else if choice.clone()[0] == "end" {
                println!("Search manually ended");
                viewing_pages = false;
            
            } else { // add an else if for if it is an integer, becuae all edge cases hit errors with just else
                for x in choice {
                    if x.parse::<usize>().is_ok() {
                        let x_num: usize = x.parse().unwrap();
                
                        app_state
                            .target_list
                            .entry(app_state.selected_target_identifier.clone())
                            .and_modify(|e| e.add_parcel(&result_vec[x_num-1]));
                    } else {
                        println!("{:?} is an incorrect selection, ignoring input", x);
                    }

                }
            }
        }      
    }
}