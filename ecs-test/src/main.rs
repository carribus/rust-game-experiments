use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Velocity {
    pub xv: f32,
    pub yv: f32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct GenerationalIndex {
    index: usize,
    generation: u64,
}

impl GenerationalIndex {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
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
}

#[derive(Debug)]
struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

#[derive(Debug)]
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        self.0.insert(index.index, Some(ArrayEntry {
            value,
            generation: index.generation,
        }));
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

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

#[derive(Debug)]
pub struct GameState {
    pub entity_allocator: GenerationalIndexAllocator,
    pub position_components: EntityMap<Position>,
    pub velocity_components: EntityMap<Velocity>,

    entities: Vec<Entity>,
}

fn main() {
    // TODO: You just added the GenerationIndex code from Kyren.github.io and were working through
    // how it should work and whether you like this approach...
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

        assert_eq!(0, e1.index());
        assert_eq!(0, e1.generation());
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

    #[test]
    fn entitymap_setget() {
        let mut a = GenerationalIndexAllocator::new();
        let mut em = GenerationalIndexArray(Vec::new());

        for i in 0..10 {
            let e1 = a.allocate();

            em.set(e1, Position { x: i as f32, y: i as f32 });
    
            assert_eq!(em.get(e1), Some(&Position {x: i as f32, y: i as f32 }));
        }
    }

    #[test]
    fn entitymap_getmut_set() {
        let mut a = GenerationalIndexAllocator::new();
        let mut em = GenerationalIndexArray(Vec::new());

        for i in 0..10 {
            let e1 = a.allocate();

            em.set(e1, Position { x: i as f32, y: i as f32 });
    
            assert_eq!(em.get(e1), Some(&Position {x: i as f32, y: i as f32 }));

            let e1 = em.get_mut(e1).unwrap();
            e1.x = (i+1) as f32;
            e1.y = (i+1) as f32;
        }

        for i in 0..10 {
            let index = GenerationalIndex{index: i, generation: 0};

            match em.get(index) {
                Some(idx) => {
                    assert_eq!(*idx, Position { x: (i+1) as f32, y: (i+1) as f32 });
                },
                None => {
                    panic!("Could not fetch entity {:?}", index);
                }
            }

        }
    }
}