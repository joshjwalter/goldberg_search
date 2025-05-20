// build a target profile that can be saved to something like a json file (idk prob not a custom file type cuz that would suck) and be able to
// resume editing and build a case on somebody with customn data gathered without automation too (eventually maybe add webscraping or find ways to automate everyting)

pub struct Target {
    first_name: String,
    realated_first_names: Vec<String>,
    last_name: String,
    address: String,
}