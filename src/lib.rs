/*!
 * Asset Allocation module
 */

pub mod engine;
pub mod model;

use toml::Table;

pub fn first() {}

pub fn importTomlDefinition(definition: String) {
    let parsed = definition
        .parse::<Table>()
        .expect("parsed asset allocation definition");

    // println!("{:?}", parsed);

    let asset_classes = linearizeDefinition(parsed);
}

fn linearizeDefinition(definition: Table) {
    // let result = vec![];

    println!("definition: {:?}", definition);

    let allocation = definition.get("Allocation").unwrap();

    // for (name, property) in allocation {
    //     // property = (name, Table)
    //     println!("property: {:?} {:?}", name, property);
    // }
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::importTomlDefinition;

    #[test]
    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml").expect("AA file read");

        importTomlDefinition(contents);

        // Assert
    }
}
