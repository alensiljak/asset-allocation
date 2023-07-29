/*!
 * Rewrite of the Asset Allocation Engine class from Cashier
 */

use std::collections::HashMap;

use crate::model::AssetClass;

pub struct AssetAllocationEngine {
    assetClassIndex: HashMap<String, AssetClass>,
    stockIndex: HashMap<String, String>
}

impl AssetAllocationEngine {
    pub fn new() -> Self {
        AssetAllocationEngine { assetClassIndex: HashMap::new(), stockIndex: HashMap::new() }
    }

    pub fn loadFullAssetAllocation(&self) {
        let assetClasses = self.loadDefinition();
        // if (!assetClasses.length) return []

        todo!()
    }

    /// Parse Asset Allocation definition.
    pub fn set_allocation(&mut self, definition: &str) {

    }

    /// Load asset allocation definition from Toml.
    fn loadDefinition(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::AssetAllocationEngine;

    #[test]
    fn test_instantiation() {
        let x = AssetAllocationEngine::new();

        assert_eq!(x.assetClassIndex.len(), 0);
        assert_eq!(x.assetClassIndex.capacity(),  0);
        assert_eq!(x.stockIndex.len(), 0);
    }

    #[test]
    fn test_full() {
        let x = AssetAllocationEngine::new();
        
        x.loadFullAssetAllocation();

        todo!()
    }

    #[test]
    fn test_setting_allocation() {
        todo!()
    }
}