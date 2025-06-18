// use arboard::Clipboard;
use rand::seq::{IndexedRandom};
use palc::Parser as _;
use palc_derive::Parser;

// At compile time, read the contents of both CSV files and embed them as static strings.
const STARS_CSV_DATA: &'static str = include_str!("data/stars.csv");
const CONSTELLATIONS_CSV_DATA: &'static str = include_str!("data/constellations.csv");

/// A helper function to parse the content of the first column from the given CSV string data.
///
/// # Arguments
/// * `csv_data` - A CSV-formatted static string obtained from the include_str! macro.
/// * `names_vec` - A mutable Vector to append the parsed names to.
fn parse_from_csv(csv_data: &'static str, names_vec: &mut Vec<String>) {
    // Create a CSV parser using the `csv` library.
    // - `has_headers(false)` indicates that our CSV files don't have header rows.
    // - `from_reader` reads from an object that implements the `Read` trait, here it's our string.
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_data.as_bytes());

    // Iterate through each row of the CSV.
    // `reader.records()` returns an iterator where each item is a Result<StringRecord, Error>.
    for result in reader.records() {
        match result {
            Ok(record) => {
                // `record.get(0)` tries to get the first field of the current row (index 0).
                // It returns an Option<&str>.
                if let Some(name) = record.get(0) {
                    let trimmed_name = name.trim();
                    // Ensure the name is not empty
                    if !trimmed_name.is_empty() {
                        names_vec.push(trimmed_name.to_string());
                    }
                }
            }
            Err(e) => {
                // If parsing a row fails, print an error warning but continue running.
                eprintln!("Warning: Failed to parse CSV row: {}", e);
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "cosmic-oracle")]
#[command(long_about = "A Cosmic Oracle that provides mystical names from stars and constellations")]
pub struct Cli {
    /// Output only the raw name without any styling or decorations
    #[arg(long, short = 'r', help = "Raw output - only the name")]
    raw: bool,
}


fn main() {
    let cli = Cli::parse();
    
    if !cli.raw {
        println!("\n✨🌟 Consulting the Cosmic Oracle for a name... 🔮⭐");
    }

    // Create an empty Vector to store names from all sources.
    let mut all_names: Vec<String> = Vec::new();

    // Call the helper function separately to parse and load star and constellation names.
    parse_from_csv(STARS_CSV_DATA, &mut all_names);
    parse_from_csv(CONSTELLATIONS_CSV_DATA, &mut all_names);

    // Check if the merged list is empty.
    if all_names.is_empty() {
        panic!("Error: The star names list is empty, unable to provide suggestions.");
    }

    let mut rng = rand::rng();

    // Randomly select a name from the complete merged list.
    if let Some(name) = all_names.choose(&mut rng) {
        if cli.raw {
            // Raw output: just the name
            print!("{}", name);
        } else {
            // Try to copy to clipboard, but don't fail if clipboard is unavailable
            // let clipboard_success = match Clipboard::new() {
            //     Ok(mut clipboard) => {
            //         match clipboard.set_text(name.clone()) {
            //             Ok(_) => true,
            //             Err(_) => false,
            //         }
            //     }
            //     Err(_) => false,
            // };

            // Display the name with cosmic styling while keeping it easy to copy.
            println!("╭─────────────────────────────────────╮");
            println!("│  🌌 The Oracle has spoken... 🌌     │");
            println!("╰─────────────────────────────────────╯");
            println!("✨ => {} <= ✨", name); // Print the name with a star prefix for easy selection and copying.
            
            // if clipboard_success {
            //     // println!("\n💫 (Copied to clipboard) 💫");
            // } else {
            //     // println!("\n⚠️  (Clipboard unavailable - please copy manually) ⚠️");
            // }
        }

    } else {
        eprintln!("Error: The stars have gone silent, unable to retrieve a name.");
    }
}