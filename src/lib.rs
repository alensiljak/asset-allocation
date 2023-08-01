/*!
 * Asset Allocation module
 */

pub mod engine;
pub mod model;

use toml::{Table, Value};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn setDefinition(definition: String) {
    let parsed = definition
        .parse::<Table>()
        .expect("parsed asset allocation definition");

    // println!("{:?}", parsed);

    let asset_classes = linearizeDefinition(parsed);
}

fn linearizeDefinition(definition: Table) {
    // let result = vec![];

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

    use crate::setDefinition;

    #[test]
    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml").expect("AA file read");

        setDefinition(contents);

        // Assert
        todo!()
    }
}
