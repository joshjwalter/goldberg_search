# Goldberg Search

A command-line tool for searching and collecting property parcel information from Stark County, Ohio's public property records database.

## Overview

Goldberg Search is a Rust-based application that interfaces with Stark County's ArcGIS REST API to search for property parcels by owner name. The tool allows users to browse search results, select relevant parcels, and build target profiles that can be saved for future reference.

## Features

- **Name-based Search**: Search property records by owner name
- **Paginated Results**: Browse through search results with customizable page sizes (up to 2000 records per page)
- **Detailed Parcel Information**: View comprehensive property details including:
  - Mailing name and address
  - Land use description
  - Allotment information
  - Billing address
  - Appraised building, land, and total values
- **Target Profiles**: Build and manage profiles by selecting multiple parcels from search results
- **JSON Export**: Save target profiles to JSON files for later use

## Dependencies

This project requires Rust to be installed. The following crates are used:

- `reqwest` (v0.12.15) - HTTP client with blocking API and JSON support
- `serde` (v1.0.219) - Serialization/deserialization framework
- `serde_json` (v1.0.140) - JSON support for serde

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system

2. Clone the repository:
   ```bash
   git clone https://github.com/joshjwalter/goldberg_search.git
   cd goldberg_search
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application:
```bash
cargo run
```

### Search Workflow

1. **Enter Target Name**: When prompted, enter the name you want to search for in "Last First" format (e.g., "Smith John")

2. **Set Results Per Page**: Choose how many results to display per page (maximum 2000)

3. **Browse Results**: Review the displayed parcel information. Each result shows:
   - Mailing name
   - Land use description
   - Allotment
   - Mailing address
   - Billing address
   - Appraised values

4. **Select Parcels**: 
   - Enter comma-separated numbers to select parcels (e.g., "1,3,5")
   - Enter "0" to view the next page of results
   - Enter "end" to finish the search

5. **Build Profile**: Selected parcels are added to your target profile for later use

## Project Structure

```
goldberg_search/
├── src/
│   ├── main.rs      # Main application logic and search interface
│   ├── target.rs    # Target profile and parcel data structures
│   └── state.rs     # State management (in development)
├── Cargo.toml       # Project dependencies and configuration
└── README.md        # This file
```

## Data Source

This application queries the Stark County, Ohio public property records database through their ArcGIS REST API endpoint:
- API: `https://scgisa.starkcountyohio.gov/arcgis/rest/services/Auditor/StarkCountyParcels_Viewer/MapServer/0/query`

## Development Status

This project is currently in active development. Some features are works in progress:

- State management functionality is being developed
- JSON export functionality is implemented but not yet integrated into the main workflow
- Additional search and filter capabilities are planned

## Contributing

This is a personal project, but suggestions and feedback are welcome through GitHub issues.

## License

Please check with the repository owner for licensing information.

## Disclaimer

This tool accesses publicly available property records. Users should ensure they comply with all applicable laws and regulations regarding the use of public records data.
