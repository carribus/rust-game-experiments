
mod gendex;

use crate::gendex::*;

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
    fn entitymap_setget() {
        let mut a = GenerationalIndexAllocator::new();
        let mut em = GenerationalIndexArray::new();

        for i in 0..10 {
            let e1 = a.allocate();

            em.set(e1, Position { x: i as f32, y: i as f32 });
    
            assert_eq!(em.get(e1), Some(&Position {x: i as f32, y: i as f32 }));
        }
    }

    #[test]
    fn entitymap_getmut_set() {
        let mut a = GenerationalIndexAllocator::new();
        let mut em = GenerationalIndexArray::new();

        for i in 0..10 {
            let e1 = a.allocate();

            em.set(e1, Position { x: i as f32, y: i as f32 });
    
            assert_eq!(em.get(e1), Some(&Position {x: i as f32, y: i as f32 }));

            let e1 = em.get_mut(e1).unwrap();
            e1.x = (i+1) as f32;
            e1.y = (i+1) as f32;
        }

        for i in 0..10 {
            let index = GenerationalIndex{ index: i, generation: 0 };

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