mod pages;
mod window;
use gtk4::{
    Application, CssProvider,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::GtkWindowExt,
};

fn main() {
    let app = Application::builder()
        .application_id("com.aurora.wallpaper")
        .build();

    app.connect_activate(|app| {
        load_css();
        let window = window::build_ui(app);
        window.present();
    });

    app.run();
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_path("styles.css");

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
