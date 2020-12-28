use gtk::prelude::*;
use gtk::{Box as GtkBox, Label, Orientation, Window, WindowType};

pub struct UI {
    label: Label,
    layout: GtkBox,
}

impl UI {
    pub fn new() -> Self {
        if gtk::init().is_err() {
            // TODO: Better error handling
            panic!("Failed to initialize GTK.");
        }

        let window = Window::new(WindowType::Toplevel);

        window.set_title("Gtone");
        window.set_default_size(350, 70);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let label = Label::new(Some("Welcome"));

        // Create a VBox with 10px spacing
        let layout = GtkBox::new(Orientation::Vertical, 10);
        layout.pack_start(&label, false, false, 0);
        window.add(&layout);
        window.show_all();

        Self { layout, label }
    }

    pub fn render(&self, note: &str) {
        self.label.set_label(note);
        self.layout.show_all();
    }

    pub fn start(&self) {
        gtk::main();
    }
}
