use std::collections::HashMap;

#[derive(Debug)]
pub struct MatrixHashTable<T> {
    data: HashMap<(usize, usize), T>,
}

impl<T> MatrixHashTable<T> {
    pub fn new() -> Self {
        MatrixHashTable {
            data: HashMap::new(),
        }
    }

    // Insert a value at a specific (row, col), overwriting any existing value
    pub fn insert(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        self.data.insert((row, col), value) // Returns the old value if there was one
    }

    // Access a value at a specific (row, col)
    pub fn get(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(&(row, col))
    }

    // Delete a value at a specific (row, col)
    pub fn delete(&mut self, row: usize, col: usize) -> Option<T> {
        self.data.remove(&(row, col))
    }

    pub fn get_keys(&self) -> Vec<(usize, usize)> {
        self.data.keys().cloned().collect()
    }
}