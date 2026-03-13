use gtk4::prelude::{BoxExt, GtkWindowExt, ListBoxRowExt, WidgetExt};

use crate::pages::appearance::build_appearance_ui;

const WINDOW_WIDTH: i32 = 750;
const WINDOW_HEIGHT: i32 = 900;

pub fn build_ui(app: &gtk4::Application) -> gtk4::ApplicationWindow {
    // Application Window
    let window = build_window(app, "Choose Wallpaper", WINDOW_WIDTH, WINDOW_HEIGHT, false);
    window.set_size_request(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Main container
    let root = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    root.add_css_class("transparent");

    // Sidebar
    let sidebar = gtk4::ListBox::new();
    sidebar.add_css_class("sidebar");
    sidebar.set_size_request(220, -1);
    sidebar.set_margin_top(12);
    sidebar.set_margin_bottom(12);

    // Content pane stack
    let stack = gtk4::Stack::new();

    // Keep adding to stack (UI) and sidebar
    stack.add_named(&build_appearance_ui(), Some("apperance"));
    sidebar.append(&build_row(
        "preferences-desktop-theme-symbolic",
        "Appearance",
    ));

    stack.add_named(
        &gtk4::Box::new(gtk4::Orientation::Horizontal, 0),
        Some("dummy"),
    );
    sidebar.append(&build_row("preferences-desktop-symbolic", "Dummy"));

    let stack_clone = stack.clone();

    sidebar.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            match row.index() {
                0 => stack_clone.set_visible_child_name("apperance"),
                1 => stack_clone.set_visible_child_name("dummy"),
                _ => {}
            }
        }
    });
    root.append(&sidebar);
    root.append(&stack);

    window.set_child(Some(&root));

    window
}

fn build_row(icon_name: &str, label_text: &str) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    //row.set_margin_top(6);
    //row.set_margin_bottom(6);
    let row_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);

    let icon = gtk4::Image::from_icon_name(icon_name);
    icon.set_pixel_size(18);
    icon.add_css_class("label");

    let label = gtk4::Label::new(Some(label_text));
    row_container.append(&icon);
    row_container.append(&label);

    row.set_child(Some(&row_container));

    row
}

fn build_window(
    app: &gtk4::Application,
    title: &str,
    default_width: i32,
    default_height: i32,
    resizable: bool,
) -> gtk4::ApplicationWindow {
    return gtk4::ApplicationWindow::builder()
        .application(app)
        .title(title)
        .default_width(default_width)
        .default_height(default_height)
        .resizable(resizable)
        .build();
}
