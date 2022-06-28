use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
//first struct
pub struct Driver {
    name : String,
    vehicle_id :String,
    license : String,
    driver_id: usize,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Parkingslot {
    locations: Vec<Driver>,
}
#[near_bindgen]
impl Parkingslot {
    pub fn empty_slots() -> Self{
        // let locations: Vec<location> = vec::empty();
        Parkingslot{
            locations: Vec::new(),
        }
    }

    pub fn parkingslot_count(&self) -> usize {
        self.locations.len()
    }

    pub fn add_driver(&mut self, name:String,
     vehicle_id:String, license:String,driver_id:usize){
        let slot1 = Driver{
            name:name.to_string(),
            vehicle_id:vehicle_id.to_string(),
            license:license.to_string(),
            driver_id:driver_id,
        };
        self.locations.push(slot1);
        env::log_str("slot available");
    }

    pub fn available_slots(&self) -> &Vec<Driver>{
        &self.locations
    }
    
    pub fn booked_slot(&mut self) {
        self.locations.pop();
        env::log_str("slot is booked");
        
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk:: AccountId;

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn free_slots(){

        let user: AccountId =AccountId::new_unchecked("masinde.test.net".to_string());
        let _context: VMContextBuilder = get_context(user.clone());

        let mut slots: Parkingslot = Parkingslot::empty_slots();
        slots.add_driver("Naivas".to_string(),"khetias".to_string(), "khetias".to_string(), 445);

        let counting: usize = slots.parkingslot_count();
        assert_eq!(counting,1)
    }


    #[test]
    fn add_slot() {
        let user: AccountId = AccountId::new_unchecked("masinde_sharrif.testnet".to_string());
        let _context: VMContextBuilder = get_context(user.clone());

        let mut slots:Parkingslot = Parkingslot::empty_slots();

        slots.add_driver("sharrif".to_string(),"KDC 146D".to_string(),"3137C23K".to_string(), 38547189);
        slots.add_driver("sharrif".to_string(),"KDD 578D".to_string(),"3142K23L".to_string(), 35627828);
        slots.add_driver("sharrif".to_string(),"KCB 192C".to_string(),"1568J76H".to_string(), 67358521);
    
        let counts:usize = slots.locations.len();
        assert_eq!(counts, 3)

    }


    // TESTS HERE
    
    //this is an example of a single thread comment
    /*this is an example of  a multi line comment*/
}
