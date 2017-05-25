extern crate gtk;
extern crate truescad;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    truescad::window::create_window();

    gtk::main();
}
