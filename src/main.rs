use ggez::event;
use ggez::ContextBuilder;
mod game;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Pong", "By God")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = game::MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
