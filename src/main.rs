use ggez::{GameError, Context, GameResult, conf, ContextBuilder, event};

struct State { }

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let state = State {};

    let config = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("maze-rs", "kevin")
        .default_conf(config)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
