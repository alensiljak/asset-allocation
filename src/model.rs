/*!
 * Definitions of the structures
 */

#[derive(Debug, Default)]
pub struct AssetClassDefinition {
    pub allocation: u8,
    pub symbols: Vec<String>
}

#[derive(Debug, Default)]
pub struct AssetClass {
  pub fullname: String,
  pub allocation: u8,
  pub allocated_value: u8,
  pub current_allocation: u8,
  pub current_value: u8,
  pub diff: u8,
  pub diff_amount: u8,
  pub diff_perc: u8,
  pub currency: String,
  pub symbols: Vec<String>,
}

impl AssetClass {
  pub fn new() -> Self {
    AssetClass::default()
  }
}

#[cfg(test)]
mod tests {
    use super::AssetClass;

  #[test]
  fn test_default() {
    let item = AssetClass::new();

    assert_eq!("".to_string(), item.fullname);
    assert_eq!(0, item.diff);
    assert_eq!(0, item.symbols.len());
  }
}