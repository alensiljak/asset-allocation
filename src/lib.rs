/*!
 * Asset Allocation module
 */

pub mod model;

use toml::Table;



pub fn first() {

}

pub fn importTomlDefinition(definition: String) {
    let parsed = definition.parse::<Table>()
        .expect("parsed asset allocation definition");
    // let parsed: AssetClassDefinition = toml::from_str(&definition).expect("Parsed Asset Class definition");

    println!("{:?}", parsed);
}

#[cfg(test)]
mod tests {
    use std::fs::{self};

    use crate::importTomlDefinition;

    #[test]
    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml")
            .expect("AA file read");
        
        importTomlDefinition(contents);

        // Assert

    }
}