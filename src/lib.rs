/*!
 * Asset Allocation module
 */

pub mod model;

pub fn first() {

}

pub fn importTomlDefinition(definition: String) {
    
}

#[cfg(test)]
mod tests {
    use std::{fs::{File, self}, io::BufReader};

    use crate::importTomlDefinition;

    fn test_import() {
        let contents = fs::read_to_string("tests/allocation.toml")
            .expect("AA file read");
        
        importTomlDefinition(contents);

        // Assert

    }
}