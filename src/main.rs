mod board;
mod control;
mod player;
mod state;

fn main() {
    let mut ctrl = control::Control::new();
    ctrl.start_loop();
}
