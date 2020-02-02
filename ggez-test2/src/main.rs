mod gendex;
use crate::gendex::*;

use ggez::*;
use rand::prelude::*;
use nalgebra as na;

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

trait System {
    fn process(ctx: &mut Context, state: &mut GameState, e: &mut Entity) -> GameResult<()>;
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct Velocity {
    xv: f32,
    yv: f32,
}

const SPRITE_SIZE: f32 = 5.0;

struct CollissionSystem;
impl System for CollissionSystem {
    fn process(ctx: &mut Context, state: &mut GameState, e: &mut Entity) -> GameResult<()> {
        let screen_rect = graphics::screen_coordinates(ctx);

        match (state.position_components.get_mut(*e), state.velocity_components.get_mut(*e)) {
            (Some(p), Some(v)) => {
                // check if we hit the bottom of the screen
                if p.y + SPRITE_SIZE >= screen_rect.h {
                    v.yv *= -0.75;
                    v.xv *= 0.8;
                }
                // check if we hit the edge of the screen
                if p.x + SPRITE_SIZE >= screen_rect.w || p.x <= 0.0 {
                    v.xv *= -0.9;
                }

                // if both velocity components are 0, the entity is dead
                if v.xv >= -0.01 && v.xv <= 0.01 {
                    state.entity_allocator.deallocate(*e);
                }
            },
            _ => (),
        }

        Ok(())
    }
}

struct MovementSystem;
impl System for MovementSystem {
    fn process(ctx: &mut Context, state: &mut GameState, e: &mut Entity) -> GameResult<()> {
        let screen_rect = graphics::screen_coordinates(ctx);
        match (state.position_components.get_mut(*e), state.velocity_components.get_mut(*e)) {
            (Some(p), Some(v)) => {
                p.x = na::clamp(p.x + v.xv, screen_rect.left(), screen_rect.w - SPRITE_SIZE);
                p.y = na::clamp(p.y + v.yv, screen_rect.top(), screen_rect.h - SPRITE_SIZE);

                v.yv = na::clamp(v.yv + 0.15, -10.0, 10.0);
                if v.yv >= -0.01 && v.yv <= 0.01 {
                    v.yv = 0.0;
                }

            },
            _ => (), // ignore if both position and velocity components are not avaialable
        }

        Ok(())
    }
}

struct RenderSystem;
impl System for RenderSystem {
    fn process(ctx: &mut Context, state: &mut GameState, e: &mut Entity) -> GameResult<()> {
        match state.position_components.get(*e) {
            Some(p) => {
                let m = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new(0.0, 0.0, SPRITE_SIZE, SPRITE_SIZE), 
                    graphics::Color::from_rgb(255, 255, 255)
                )?;

                graphics::draw(ctx, &m, (na::Point2::new(p.x, p.y),))?;
            },
            None => (),
        }

        Ok(())
    }
}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GameState {
    pub entity_allocator: GenerationalIndexAllocator,
    pub entities: Rc<RefCell<Vec<Entity>>>,
    pub position_components: EntityMap<Position>,
    pub velocity_components: EntityMap<Velocity>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            entity_allocator: GenerationalIndexAllocator::new(),
            entities: Rc::new(RefCell::new(Vec::new())),
            position_components: EntityMap::new(),
            velocity_components: EntityMap::new(),
        }
    }

    fn draw_debug_info(&self, ctx: &mut Context) -> GameResult<()> {
        let tf = graphics::TextFragment::new(format!("fps={:.0}, live_entities: {} / {}", 
            timer::fps(ctx), 
            self.entity_allocator.live_entity_count(),
            self.entity_allocator.allocated_entity_count())
        );
        let text = graphics::Text::new(tf);

        graphics::clear(ctx, graphics::Color::from_rgb(32, 32, 48));
        graphics::draw(ctx, &text, (nalgebra::Point2::new(10.0, 10.0),))?;

        Ok(())
    }

    fn generate_entities(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let e = self.entity_allocator.allocate();
            self.entities.borrow_mut().insert(e.index, e);
            self.position_components.set(e, Position{ x: rng.gen::<f32>() * 1280.0, y: rng.gen::<f32>() * 900.0 });
            self.velocity_components.set(e, Velocity{ xv: 1.0 + rng.gen::<f32>() * 5.0, yv: 0.0 });
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // run various systems
        let entities = self.entities.clone();

        for e in entities.borrow_mut().iter_mut() {
            if self.entity_allocator.is_live(*e) {
                MovementSystem::process(ctx, self, e)?;
                CollissionSystem::process(ctx, self, e)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(20, 40, 80));

        self.draw_debug_info(ctx)?;

        let entities = self.entities.clone();

        for e in entities.borrow_mut().iter_mut() {
            if self.entity_allocator.is_live(*e) {
                RenderSystem::process(ctx, self, e)?;
            }
        }


        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.generate_entities();
    }
}

fn main() {
    let state = &mut GameState::new();
    let mut c = conf::Conf::new();
    c.window_mode.width = 1280.0;
    c.window_mode.height = 1024.0;
    c.window_setup.title = String::from("ggez game test 2");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("ggez game test 2", "Peter Mares")
        .conf(c)
        .build()
        .expect("Failed to create ggez context!");

    // setup the immovable entity
    // let e = state.entity_allocator.allocate();
    // state.entities.borrow_mut().insert(e.index, e);
    // state.position_components.set(e, Position{ x: 640.0, y: 512.0 });
    
    // setup the movable entity
    state.generate_entities();



    match event::run(ctx, event_loop, state) {
        Ok(_) => (),
        Err(e) => println!("ERROR: {}", e),
    }
}
