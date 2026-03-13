use std::{fs, path::PathBuf};

use gtk4::{
    DropDown, GridView, Picture, SignalListItemFactory, StringList,
    gio::prelude::ListModelExt,
    glib::object::{Cast, CastNone},
    prelude::{BoxExt, ListItemExt, WidgetExt},
};

const CONTAINER_MARGIN: i32 = 24;
const CONTAINER_SPACING: i32 = 16;

pub fn build_appearance_ui() -> gtk4::Box {
    // Top level container
    let container = build_container(CONTAINER_MARGIN, CONTAINER_SPACING);
    //window.set_child(Some(&container));

    // Add labels for heading & subheadings
    container.append(&build_labels("Personalization", "title-2"));
    container.append(&build_labels("Wallpaper", "title-3"));

    // Preview Image
    let preview = gtk4::Picture::builder()
        .height_request(250)
        .vexpand(false)
        .css_classes(["preview"])
        .can_shrink(true)
        .keep_aspect_ratio(false)
        .build();
    container.append(&preview);

    // Wallpaper preview startegy
    let layout_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);

    let layout_label = gtk4::Label::new(Some("Wallpaper Layout"));

    let layout_dropdown = gtk4::DropDown::from_strings(&["Fill", "Fit", "Center", "Tile"]);

    layout_box.append(&layout_label);
    layout_box.append(&layout_dropdown);

    container.append(&layout_box);

    // Wallpaper view
    let scroll_view = gtk4::ScrolledWindow::builder()
        .hexpand(true)
        .height_request(450)
        .overflow(gtk4::Overflow::Hidden)
        .build();

    // Wallpaper chooser grid view
    let grid_view = build_grid_view(&preview, 3);
    scroll_view.set_child(Some(&grid_view));
    container.append(&scroll_view);

    // Section Separator
    let separator = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    container.append(&separator);

    // Rotate Wallpaper section

    // Interval section
    let interval_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let interval_label = build_labels("Change every", "");
    let interval_dropdown =
        gtk4::DropDown::from_strings(&[" 5 minutes", "10 minutes", "30 minutes", "1 hour"]);

    interval_dropdown.set_sensitive(false);
    interval_container.append(&interval_label);
    interval_container.append(&interval_dropdown);

    container.append(&build_rotatation_section(&interval_dropdown));
    container.append(&interval_container);

    container
}

fn build_container(margin: i32, spacing: i32) -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, spacing);

    container.set_margin_bottom(margin);
    container.set_margin_end(margin);
    container.set_margin_start(margin);
    container.set_margin_top(margin);

    container
}

fn build_labels(label_text: &str, label_class: &str) -> gtk4::Label {
    let label = gtk4::Label::builder()
        .label(label_text)
        .halign(gtk4::Align::Start)
        .css_classes([label_class])
        .build();

    label
}

fn build_grid_view(preview_image: &Picture, min_columns: u32) -> GridView {
    let grid_view = GridView::builder()
        .model(&build_selection(&preview_image))
        .factory(&build_factory())
        .min_columns(min_columns)
        .build();

    grid_view
}

fn build_rotatation_section(drop_down: &DropDown) -> gtk4::Box {
    let section_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    let section_label = build_labels("Rotate Wallpapers", "");

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    let rotation_toggle = gtk4::Switch::new();

    section_container.append(&section_label);
    section_container.append(&spacer);
    section_container.append(&rotation_toggle);

    let dropdown_clone = drop_down.clone();
    rotation_toggle.connect_active_notify(move |s| {
        dropdown_clone.set_sensitive(s.is_active());
    });

    section_container
}

fn build_selection(preview: &Picture) -> gtk4::SingleSelection {
    let list = gtk4::StringList::new(&[]);
    let selection = gtk4::SingleSelection::new(Some(list.clone()));

    let preview_clone = preview.clone();

    selection.connect_selected_notify(move |selected| {
        let index = selected.selected();
        if index == gtk4::INVALID_LIST_POSITION {
            return;
        }
        let path = selected
            .model()
            .unwrap()
            .item(index)
            .and_downcast::<gtk4::StringObject>()
            .unwrap()
            .string();

        if path != "__add__" {
            preview_clone.set_filename(Some(path.as_str()));
        }
    });

    // Load images from Pictures dir
    load_thumbnails(&list);

    selection
}

fn build_factory() -> SignalListItemFactory {
    let factory = gtk4::SignalListItemFactory::new();

    factory.connect_setup(|_, item| {
        let item = item.downcast_ref::<gtk4::ListItem>().unwrap();

        let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        container.set_size_request(150, 90);
        container.set_css_classes(&["wallpaper-tile"]);
        container.set_overflow(gtk4::Overflow::Hidden);

        let picture = gtk4::Picture::new();
        picture.set_size_request(-1, 90);
        picture.set_can_shrink(true);
        picture.set_keep_aspect_ratio(false);

        container.append(&picture);

        item.set_child(Some(&container));
    });

    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk4::ListItem>().unwrap();

        let container = item.child().and_downcast::<gtk4::Box>().unwrap();

        let obj = item.item().and_downcast::<gtk4::StringObject>().unwrap();

        let path = obj.string();

        // remove previous child
        while let Some(child) = container.first_child() {
            container.remove(&child);
        }

        // All of this just to display the + button
        if path == "__add__" {
            let center = gtk4::CenterBox::new();
            center.set_hexpand(true);
            center.set_vexpand(true);

            let icon = gtk4::Image::from_icon_name("list-add-symbolic");
            icon.set_pixel_size(32);
            center.set_center_widget(Some(&icon));
            container.append(&center);
        } else {
            let picture = gtk4::Picture::for_filename(path.as_str());
            picture.set_size_request(-1, 90);
            picture.set_can_shrink(true);
            picture.set_keep_aspect_ratio(false);

            container.append(&picture);
        }
    });

    factory
}

fn load_thumbnails(list_of_images: &StringList) {
    let pictures_dir = get_pictures_dir();

    if let Ok(entries) = fs::read_dir(pictures_dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "png" || ext == "jpg" || ext == "jpeg" {
                    list_of_images.append(&path.to_string_lossy());
                }
            }
        }
    }

    // This is for the + button
    list_of_images.append("__add__");
}

fn get_pictures_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    PathBuf::from(home).join("Pictures")
}
