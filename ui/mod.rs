use gtk::prelude::*;
use gtk::{Box as GtkBox, Label, Orientation, Window, WindowType};

pub struct UI {
    label: Label,
    window: Window,
}

impl UI {
    pub fn new() -> Self {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let window = Window::new(WindowType::Toplevel);

        window.set_title("Gtone");
        window.set_default_size(350, 70);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let label = Label::new(Some("D#"));

        // Create a VBox with 10px spacing
        let bx = GtkBox::new(Orientation::Vertical, 10);
        bx.pack_start(&label, false, false, 0);
        window.add(&bx);
        window.show_all();

        Self { window, label }
    }

    pub fn render(&self, note: &str) {
        self.label.set_label(note);
        self.window.show_all();
    }

    pub fn start(&self) {
        gtk::main();
    }
}
