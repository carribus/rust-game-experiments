use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct GenerationalIndex {
    pub index: usize,
    pub generation: u64,
}

#[derive(Debug, Copy, Clone, Default)]
struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}

#[derive(Debug, Clone, Default)]
pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: VecDeque<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        GenerationalIndexAllocator {
            entries: Vec::new(),
            free: VecDeque::new(),
        }
    }

    pub fn allocate(&mut self) -> GenerationalIndex {
        match self.free.pop_front() {
            Some(index) => {
                // free index found, use it
                self.entries[index].is_live = true;
                self.entries[index].generation += 1;
                GenerationalIndex {
                    index,
                    generation: self.entries[index].generation,
                }
            },
            None => {      
                // no free index, create a new one
                self.entries.push(AllocatorEntry {
                    is_live: true,
                    generation: 0,
                });
                GenerationalIndex {
                    index: self.entries.len()-1,
                    generation: 0,
                }
            }
        }
    }

    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if self.entries[index.index].generation == index.generation && 
           self.entries[index.index].is_live == true 
        {
            self.entries[index.index].is_live = false;
            self.free.push_back(index.index);
            true
        } else {
            false
        }
    }

    pub fn is_live(&self, index: GenerationalIndex) -> bool {
        self.entries[index.index].is_live
    }

    pub fn live_entity_count(&self) -> usize {
        self.entries.len() - self.free.len()
    }

    pub fn allocated_entity_count(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Debug)]
struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

#[derive(Debug)]
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn new() -> Self {
        GenerationalIndexArray(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        if self.0.len() > index.index {
            self.0[index.index] = Some(ArrayEntry {
                value,
                generation: index.generation,
            });
        } else {
            // if the index is past the length of the current vec, we need to add some None elements
            while self.0.len() <= index.index { self.0.push(None) }
            self.0.insert(index.index, Some(ArrayEntry {
                value,
                generation: index.generation,
            }));
        }
    }

    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        if let Some(e) = self.0.get(index.index) {
            match e {
                Some(ref entry) => {
                    if entry.generation == index.generation {
                        Some(&entry.value)
                    } else {
                        None
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        if let Some(e) = self.0.get_mut(index.index) {
            match e {
                Some(ref mut entry) => {
                    if entry.generation == index.generation {
                        Some(&mut entry.value)
                    } else {
                        None
                    }
                },
                None => None,
            }
        } else {
            None
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn genindex_equality_test() {
        let e1 = GenerationalIndex{ index: 0, generation: 0 };
        
        assert!(e1 == GenerationalIndex{ index: 0, generation: 0 });
        assert!(e1 != GenerationalIndex{ index: 0, generation: 1 });
        assert!(e1 != GenerationalIndex{ index: 1, generation: 0 });
    }

    #[test]
    fn genindex_getters_test() {
        let e1 = GenerationalIndex{ index: 0, generation: 0 };

        assert_eq!(0, e1.index);
        assert_eq!(0, e1.generation);
    }

    #[test]
    fn allocator_create_entity() {
        let mut a = GenerationalIndexAllocator::new();
        let entity = a.allocate();

        assert_eq!(0, entity.index);
        assert_eq!(0, entity.generation);
    }

    #[test]
    fn allocator_create_two_entities() {
        let mut a = GenerationalIndexAllocator::new();
        let e1 = a.allocate();
        let e2 = a.allocate();

        assert_eq!(0, e1.index);
        assert_eq!(0, e1.generation);
        assert_eq!(1, e2.index);
        assert_eq!(0, e2.generation);
    }

    #[test]
    fn allocator_recreate_entity() {
        let mut a = GenerationalIndexAllocator::new();
        let e1 = a.allocate();

        assert!(a.deallocate(e1) == true);

        let e1 = a.allocate();

        assert_eq!(0, e1.index);
        assert_eq!(1, e1.generation);
    }

    #[test]
    fn allocator_recreate_entity_middle() {
        let mut a = GenerationalIndexAllocator::new();
        let mut e_vec = vec![a.allocate(), a.allocate(), a.allocate()];
        
        assert!(a.deallocate(e_vec[1]) == true);
        assert!(a.deallocate(e_vec[1]) == false);

        e_vec[1] = a.allocate();

        assert_eq!(1, e_vec[1].index);
        assert_eq!(1, e_vec[1].generation);
    }
}