/*!
 * Asset Allocation module
 */

pub mod engine;
pub mod model;

use model::AssetClass;
use toml::{value::Array, Table, Value};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn loadDefinition(definition: String) -> Vec<AssetClass> {
    let allocation = definition
        .parse::<Table>()
        .expect("parsed asset allocation definition");

    // println!("{:?}", parsed);

    let asset_classes = linearizeDefinition(&allocation);

    // let result = await this.validateAndSave(assetClasses)

    asset_classes
}

fn linearizeDefinition(root: &Table) -> Vec<AssetClass> {
    let mut result: Vec<AssetClass> = vec![];

    // process the children.
    for (key, value) in root {
        if value.is_table() {
            // add the root class
            let ac = get_asset_class_from_toml(key, value.as_table().unwrap());
            result.push(ac);

            // add to the list
            let children = linearizeDefinition(value.as_table().unwrap());
            for child in children {
                result.push(child);
            }
        }
    }

    result
}

fn get_asset_class_from_toml(name: &String, value: &Table) -> AssetClass {
    let mut ac = AssetClass::new();
    ac.fullname = name.to_owned();

    // get the allocation
    let mut node = &value["allocation"];
    let allocation = node.as_integer().unwrap();
    ac.allocation = allocation as u8;

    // symbols
    if value.contains_key("symbols") {
        node = &value["symbols"];
        if let Some(symbols) = node.as_array() {
            // Symbols are Strings
            for symbol_val in symbols {
                ac.symbols.push(symbol_val.to_string());
            }
        }
    }

    ac
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::loadDefinition;

    #[test]
    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml").expect("AA file read");

        let allocation = loadDefinition(contents);

        // Assert
        assert_eq!(7, allocation.len());
    }
}
