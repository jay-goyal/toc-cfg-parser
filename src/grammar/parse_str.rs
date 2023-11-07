use std::collections::HashMap;

use super::{Grammar, GrammarRule};

impl Grammar {
    pub fn get_parsing_table(&self) -> HashMap<(char, char), GrammarRule> {
        for rule in self.rules {}
        HashMap::new()
    }
}
