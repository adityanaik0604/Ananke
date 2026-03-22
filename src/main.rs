mod cpu;
mod memory;
mod network;
mod process;
mod ui;

fn main() {
    if let Err(err) = ui::run_app() {
        println!("Error: {}", err);
    }
}
