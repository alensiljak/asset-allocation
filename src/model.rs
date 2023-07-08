/*!
 * Definitions of the structures
 */



#[derive(Debug)]
pub struct AssetClassDefinition {
    pub allocation: u8,
    pub symbols: Vec<String>
}

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