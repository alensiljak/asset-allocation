/*!
 * Asset Allocation module
 */

pub mod engine;
pub mod model;

use model::AssetClass;
use toml::{Table, Value};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn loadDefinition(definition: String) -> Vec<AssetClass> {
    let allocation = definition
        .parse::<Table>()
        .expect("parsed asset allocation definition");

    // println!("{:?}", parsed);

    let asset_classes = linearizeDefinition(allocation);

    // let result = await this.validateAndSave(assetClasses)

    todo!()
}

fn linearizeDefinition(definition: Table) {
    let mut result: Vec<AssetClass> = vec![];

    println!("definition: {:?}", definition);

    let allocation = definition.get("Allocation").unwrap().as_table().unwrap();
    println!("allocation: {:?}", allocation);

    for (name, property) in allocation {
        if property.is_table() {
            // todo: add to the list
            // get_linear_asset_classes(property.as_table());
        } else {
            let mut ac = AssetClass::new();
            ac.fullname = name.to_owned();
            ac.allocation = property.as_integer().unwrap() as u8;

            println!("added: {:?}", ac);

            result.push(ac);
        }
    }
}

fn get_linear_asset_classes(root: Table) {
    // return only the children
    for (key, value) in root {
        println!("{:?} {:?}", key, value);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::loadDefinition;

    #[test]
    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml").expect("AA file read");

        loadDefinition(contents);

        // Assert
        todo!()
    }
}
