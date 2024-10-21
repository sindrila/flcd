use crate::domain::adt::hash_table::HashTable;
pub struct SymbolTable {
    hash_table: HashTable<String, i32>,
    last_index: i32,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            hash_table: HashTable::new(),
            last_index: -1,
        }
    }

    pub fn len(&self) -> usize {
        self.hash_table.len()
    }

    pub fn insert_symbol(&mut self, token: String) {
        if self.contains_token(token.clone()) {
            return;
        }
        self.last_index += 1;
        self.hash_table.insert(token, self.last_index);
    }

    pub fn get_position_of_symbol(&self, token: String) -> Option<&i32> {
        self.hash_table.get(&token)
    }

    pub fn get_symbol_at_position(&self, given_position: &i32) -> Option<&String> {
         for (token, position) in self.hash_table.iter() {
             if position ==  given_position{
                 return Some(token);
             }
         }
        None
    }

    pub fn remove(&mut self, key: String) {
        self.hash_table.remove(&key);
    }

    pub fn contains_token(&self, token: String) -> bool {
        self.hash_table.contains_key(&token)
    }

}
