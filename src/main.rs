use rusty_chess::game::Game;

fn main() {
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    let mut app = Game::init(1280, 960, "Rusty Chess");
    app.run();
}