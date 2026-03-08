mod window_manager;

use gtk4::{Application, ApplicationWindow, Box as GtkBox, Button, Image, Orientation};
use gtk4::{glib, prelude::*};
use gtk4_layer_shell::LayerShell;
use std::path::Path;
use std::process::Command;

fn main() {
    let _ = glib::MainContext::default();

    let app = Application::builder()
        .application_id("com.aurora.dock")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder().application(app).build();

        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Top);
        window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        window.set_margin(gtk4_layer_shell::Edge::Bottom, 12);
        window.set_exclusive_zone(-1);
        window.set_decorated(false);

        let container = GtkBox::new(Orientation::Horizontal, 16);
        container.set_margin_top(10);
        container.set_margin_bottom(10);
        container.set_margin_start(20);
        container.set_margin_end(20);

        container.add_css_class("dock-container");

        let buttons_map: std::collections::HashMap<String, Button> =
            std::collections::HashMap::new();

        let default_apps = [
            ("firefox", "firefox"),
            ("utilities-terminal", "konsole"),
            ("system-file-manager", "dolphin"),
        ];

        for (icon_name, command) in default_apps {
            let image = Image::from_icon_name(icon_name);
            image.set_pixel_size(48);

            let button = Button::builder().child(&image).build();
            button.add_css_class("flat");

            let cmd = command.to_string();

            button.connect_clicked(move |_| {
                Command::new(&cmd).spawn().expect("Failed to launch app");
            });

            container.append(&button);
        }

        let buttons_for_task = buttons_map.clone();
        glib::timeout_add_local(std::time::Duration::from_secs(2), move || {
            println!("--- Timer Tick ---");

            // We can just iterate directly now!
            for (app_id, button) in &buttons_for_task {
                if window_manager::is_app_running(app_id) {
                    button.add_css_class("active");
                } else {
                    button.remove_css_class("active");
                }
            }

            glib::ControlFlow::Continue
        });

        window.set_child(Some(&container));

        // CSS
        let css = gtk4::CssProvider::new();
        css.load_from_path(Path::new("resources/style.css"));

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        window.present();
    });
    app.run();
}
