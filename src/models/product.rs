use cuid::cuid2;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub description: String,
    
}

impl Product {
    pub fn new(name: &str, price: f64, description: &str) -> Result<Self, String> {
        if name.is_empty(){
            return Err("Produc name cannot be empty".to_string());
        }
        if price < 0.0 {
            return Err("Price cannot be negative".to_string());
        }
        if description.len()> 100 && description.len() > 0 {
            return Err("Description length must be maximum 100 characters long".to_string())
        }
      
        Ok(Self {
            id: cuid2(),
            name: name.to_string(),
            price,
            description: description.to_string(),
            
        })
    }

    // Getter Methods (Accessors)

    // Here I am using the referencing operator so that we get
    //we borrow the original value but as READ-ONLY, therefore making this a getter method

    // Get Product Id
    pub fn id(&self) -> &str {
        &self.id
    }

    //Get Prduct Name
    pub fn name(&self) -> &str {
        &self.id
    }

    //Get Product Price
    pub fn price(&self) -> f64 {
        self.price
    }
    //Get Description
    pub fn description(&self) -> &str {
        &self.description
    }

    //
}

// Default impl for Product
impl Default for Product {
    fn default() -> Self {
        Self {
            id: cuid2(),
            name: "Default Product".to_string(),
            price: 0.0,
            description: "None".to_string()
        }
    }
}