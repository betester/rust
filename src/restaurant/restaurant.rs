
use super::{customer::{Customer}, menu::Menu, chef::Chef, food::Food};
#[derive(Debug)]
pub struct Restaurant<'a> {
    pub name : String,
    pub adress : String,
    pub rating : u8,
    pub available_seats : usize,
    pub customers : Vec<&'a Customer<'a>>,
    pub chefs : Vec<&'a Chef>,
    pub menu : Menu,
}

impl<'a> Restaurant<'a> {

    pub fn add_food_menu(&mut self,food: Food) {
        self.menu.add_food(food);
    }

    pub fn remove_food_menu(&mut self,food: Food) {
        self.menu.remove_food(food)
    }

    pub fn add_customers(&mut self, customer : &'a Customer<'a>) {
        if self.is_visitable() {
            self.customers.push(customer);
        }
    }

    pub fn remove_customers(&mut self, username : String ) {
        match self.get_customer_by_username(username) {
            Ok(chef) => {self.customers.remove(chef);},
            Err(err_msg) => println!("{}",err_msg)
        }
    }

    pub fn add_chefs(&mut self, chef : &'a Chef) {
        self.chefs.push(chef);
    }

    pub fn remove_chefs(&mut self, chef_id : u32) {
        match self.get_chef_by_id(chef_id) {
            Ok(chef) => {self.chefs.remove(chef);},
            Err(err_msg) => println!("{}",err_msg)
        }
    }

    pub fn get_chef_by_id(&self, chef_id : u32 ) -> Result<usize,String> {
        for index in 0..self.chefs.len() {
            if self.chefs.get(index).unwrap().chef_id == chef_id {
                return Ok(index);
            }
        }
        return Err("Chef cannot be found".to_string())
    }

    pub fn get_customer_by_username(&self, username : String) -> Result<usize,String> {
        for index in 0..self.customers.len() {
            if self.customers.get(index).unwrap().username == username {
                return Ok(index);
            }
        }
        return Err("Customer cannot be found".to_string());
    }

    pub fn handle_order(&mut self, 
                        customer : &mut Customer, 
                        ordered_food_name : &[String]) {
        for food_name in ordered_food_name {
            customer.order_menu(food_name, &self.menu);
        }
    }


    pub fn is_visitable(&self) -> bool {
        true
    }


    

}