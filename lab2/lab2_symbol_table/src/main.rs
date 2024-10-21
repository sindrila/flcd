mod domain;

use crate::domain::symbol_table;
fn main() {
    // let mut my_hashmap: test::HashMap<i32, String> = test::HashMap::new();
    let mut symbol_table = symbol_table::SymbolTable::new();
    symbol_table.insert_symbol("alex".to_string());
    assert_eq!(symbol_table.len(), 1); // length test
    symbol_table.insert_symbol("alexutz".to_string());
    println!(
        "{}",
        symbol_table
            .get_position_of_symbol("alexutz".to_string())
            .unwrap()
    );
    assert_eq!(
        symbol_table.get_position_of_symbol("alexutz".to_string()),
        Some(&1)
    ); // ascending order test
    assert_eq!(
        symbol_table.get_position_of_symbol("nuexista".to_string()),
        None
    ); // nonexistent token test
    assert_eq!(
        symbol_table.get_symbol_at_position(
            symbol_table
                .get_position_of_symbol("alex".to_string())
                .unwrap()
        ),
        Some("alex".to_string()).as_ref()
    ); // symbol at position test
    symbol_table.remove("new value".to_string());
    assert_eq!(
        symbol_table.get_position_of_symbol("new value".to_string()),
        None
    );
}
