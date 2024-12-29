use print_manager;
use serde_json;
use std::fs;


fn main(){
    print!("Test main")
    match print_manager::get_all_printers() {
        Ok(json_data) => println!("{:?}", serde_json::to_string(&json_data).unwrap()),
        Err(err) => println!("{:?}", err), 
    }
    
    // match print_manager::print_file("00057.SPL", "receipt") {
    //     Ok(_) => println!("Printout successful"),
    //     Err(e) => println!("Error: {}", e),
    // }

    // let file_path = "00057.SPL";

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // match print_manager::print_data(&contents, "receipt") {
    //     Ok(_) => println!("Printout successful"),
    //     Err(e) => println!("Error: {}", e),
    // }
}
