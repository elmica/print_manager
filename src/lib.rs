use printers::common::base::printer::Printer;
use serde_json::{json, Value};


/// Gets json array of printers
/// 
///Can return the following error:
///     
///"No printers found."
pub fn get_all_printers() -> Result<Value, String> {
    let printers_result = printers::get_printers();

    if printers_result.is_empty() {
        return Err("No printers found.".to_string());
    }

    let printers_json: Vec<Value> = printers_result.iter()
    .map(|printer| json!({
        "name": printer.name,
        "system_name": printer.system_name,
        "is_default": printer.is_default,
        "uri": printer.uri,
        "port_name": printer.port_name,
        "is_shared": printer.is_shared,
        "location": printer.location,
        "driver_name": printer.driver_name,
        "processor": printer.processor,
        "data_type": printer.data_type,
        "description": printer.description,
    }))
    .collect();

    Ok(Value::Array(printers_json))
}


/// Prints a zpl or esc file to specific path with a given printer:
/// 
/// Can be called like this:
///   
///     match print_manager::print_file("00057.SPL", "receipt") { 
///         Ok(_) => println!("Printout successful"),
///         Err(e) => println!("Error: {}", e),
///         }
/// 
///Can return the following errors: 
/// 
///"Failed to print file: {}"
///"Printer '{}' not found"
///     
pub fn print_file(file_path: &str, printer_name: &str) -> Result<(), String> {
    let myprinter: Option<Printer> = printers::get_printer_by_name(printer_name);

    if let Some(printer) = myprinter {
        match printer.print_file(file_path, None) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to print file: {}", e)),

        }
    } else {
        Err(format!("Printer '{}' not found", printer_name))
    }
}


/// prints a zpl or esc file to given printer:
/// 
/// Can be called like this:
/// 
///     match print_manager::print_data(&contents, "receipt") {
///       Ok(_) => println!("Printout successful"),
///       Err(e) => println!("Error: {}", e),
///       }
/// 
///Can return the following errors: 
/// 
///"Failed to print data: {}"
///"Printer '{}' not found"
///     
pub fn print_data(data: &str, printer_name: &str) -> Result<(), String> {
    let myprinter: Option<Printer> = printers::get_printer_by_name(printer_name);

    if let Some(printer) = myprinter {
        match printer.print(data.as_bytes(), Some("CHS printout")) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to print data: {}", e)),
        }
    } else {
        Err(format!("Printer '{}' not found", printer_name))
    }
}
