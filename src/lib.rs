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
    let result: Vec<AssetClass> = vec![];

    println!("definition: {:?}", definition);

    let allocation: &Value = definition.get("Allocation").unwrap();
    println!("allocation: {:?}", allocation);

    // for (name, property) in allocation {
    //     // property = (name, Table)
    //     println!("property: {:?} {:?}", name, property);
    // }
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
