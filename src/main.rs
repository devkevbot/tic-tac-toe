mod board;
mod control;
mod player;
mod state;

use control::Control;

fn main() {
    let mut ctrl = Control::new();
    ctrl.start_loop();
}
