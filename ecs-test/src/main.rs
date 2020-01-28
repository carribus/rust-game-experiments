
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
    pub entities: Vec<Entity>,
    pub position_components: EntityMap<Position>,
    pub velocity_components: EntityMap<Velocity>,
}

struct AutoMovementSystem {}

impl AutoMovementSystem {
    fn process(state: &mut GameState) {
        for e in state.entities.iter() {
            let v = state.velocity_components.get(*e);
            let p = state.position_components.get_mut(*e);

            match (p, v) {
                (Some(p), Some(v)) => {
                    p.x += v.xv;
                    p.y += v.yv;
                },
                _ => (),
            }
        }
    }
}

fn main() {
    let mut state = GameState {
        entity_allocator: GenerationalIndexAllocator::new(),
        entities: Vec::new(),
        position_components: EntityMap::new(),
        velocity_components: EntityMap::new(),
    };

    for i in 0..10 {
        let e = state.entity_allocator.allocate();
        state.entities.insert(e.index, e);
        state.position_components.set(e, Position{ x: i as f32, y: i as f32 });
        state.velocity_components.set(e, Velocity{ xv: (i+1) as f32, yv: (i+1) as f32 });
    }

    // do 10 iterations of movement
    for i in 0..10 {
        AutoMovementSystem::process(&mut state);
        println!("Tick {}:\n", i);
        for (idx, e) in state.entities.iter().enumerate() {
            if let Some(p) = state.position_components.get(*e) {
                println!("[{}] x:{:.1}, y:{:.1}", idx, p.x, p.y);
            }
        }
        println!("");
    }
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