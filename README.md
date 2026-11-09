# Goldberg Search

Goldberg Search is a small Rust CLI tool for searching Stark County, Ohio parcel records by owner name and reviewing matches in paginated results.

## Features

- Search parcel records by owner name
- Paginate through result sets
- View key parcel details in the terminal
- Select likely matches and store them in an in-memory target profile

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Internet access (the tool queries a live Stark County GIS API)

## Installation

1. Clone the repository.
2. Build the project:

```bash
cargo build --release
```

## Usage

Run the CLI:

```bash
cargo run
```

You will be prompted to:

1. Enter a target name (`Last First` format works best)
2. Choose a max results-per-page value
3. Review records and select candidate entries by number, comma-separated
4. Enter `0` for the next page or `end` to stop

## Data Source

This project queries public parcel data from:

- `https://scgisa.starkcountyohio.gov/arcgis/rest/services/Auditor/StarkCountyParcels_Viewer/MapServer/0/query`

## Development

### Build

```bash
cargo build --verbose
```

### Test

```bash
cargo test --verbose
```

## Project Status

This is an early-stage single-creator tool and is under active iteration.

## Contributing

Issues and pull requests are welcome.

## License

No license file is currently included in this repository.
