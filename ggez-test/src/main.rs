use ggez::*;

struct State {
    dt: std::time::Duration,
    pos_x: f32,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        let tf = graphics::TextFragment::new(format!("Hello ggez! dt = {}ms, fps={:.0}", self.dt.subsec_millis(), timer::fps(ctx)));
        let text = graphics::Text::new(tf);

        graphics::clear(ctx, graphics::Color::from_rgb(32, 32, 48));
        graphics::draw(ctx, &text, (nalgebra::Point2::new(10.0, 10.0),))?;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (nalgebra::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = &mut State { 
        dt: std::time::Duration::new(0, 0),
        pos_x: 0.0,
    };

    let c = create_config();

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "Peter Mares")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}

fn create_config() -> conf::Conf {
    let mut c = conf::Conf::new();
    c.window_mode.width = 1024.0;
    c.window_mode.height = 768.0;
    c.window_setup.title = "ggez Test 1".to_string();

    c
}