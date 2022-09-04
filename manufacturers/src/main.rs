#![deny(clippy::all)]

use std::env;

const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

struct Manufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or_default().contains(needle)
            || self.common_name.unwrap_or_default().contains(needle)
            || self.country.unwrap_or_default().contains(needle)
    }
}

impl<'a> Manufacturer<'a> {
    fn description(&self) -> String {
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();
        format!(
            "\tName: {}\n\tCommon name: {}\n\tCountry: {}",
            name, common_name, country
        )
    }
}

// Make main async
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all arguments
    let args: Vec<String> = env::args().collect();

    // Abort if no argument
    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }
    let keyword = &args[1];

    let client = reqwest::Client::new();
    let res = client
        .get(API_URL)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:?}", res.as_object().unwrap());
    let manufacturer_json = res
        .as_object()
        .unwrap()
        .iter()
        .find(|(key, _)| *key == "Results")
        .unwrap()
        .1
        .as_array()
        .unwrap()
        .iter();
    let manufacturers = manufacturer_json.map(|json| {
        let object = json.as_object().unwrap();
        let name = object.get("Mfr_Name").unwrap().as_str();
        let common_name = object.get("Mfr_CommonName").unwrap().as_str();
        let country = object.get("Country").unwrap().as_str();
        Manufacturer {
            name,
            common_name,
            country,
        }
    });
    let founded = manufacturers
        .filter(|manufacturer| manufacturer.contains(keyword))
        .collect::<Vec<_>>();
    if founded.is_empty() {
        return Err("Not found".into());
    } else {
        println!("We found:");
        for man in founded {
            println!("\n{}", man.description());
        }
    }

    Ok(())
}
