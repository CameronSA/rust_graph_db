use std::collections::HashMap;

pub struct EntityMap<T> {
    entities: HashMap<usize, T>,
}

impl<T> EntityMap<T> {
    pub fn new() -> EntityMap<T> {
        let entities = HashMap::new();
        EntityMap { entities }
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn push(&mut self, entity: T) -> usize {
        let mut index = 0;
        if let Some(i) = self.entities.keys().max() {
            index = *i + 1;
        }

        self.entities.insert(index, entity);
        index
    }

    pub fn get_indices(&self) -> Vec<&usize> {
        self.entities.keys().collect::<Vec<_>>()
    }

    pub fn get(&self, index: &usize) -> Option<&T> {
        self.entities.get(index)
    }

    pub fn get_mut(&mut self, index: &usize) -> Option<&mut T> {
        self.entities.get_mut(index)
    }

    pub fn remove(&mut self, index: &usize) -> Option<T> {
        self.entities.remove(index)
    }

    pub fn entities(&self) -> &HashMap<usize, T> {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut HashMap<usize, T> {
        &mut self.entities
    }
}
