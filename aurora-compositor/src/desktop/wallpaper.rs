use std::{fs, path::PathBuf};

use image::ImageReader;
use smithay::backend::{
    allocator::Fourcc,
    renderer::{
        ImportMem,
        element::texture::TextureBuffer,
        gles::{GlesRenderer, GlesTexture},
    },
};

use crate::state::Aurora;

pub fn scan_for_wallpapers(dir: &str) -> Vec<PathBuf> {
    let mut wallpapers = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "jpg" || ext == "jpeg" || ext == "png" {
                    wallpapers.push(path);
                }
            }
        }
    }
    wallpapers
}

pub fn load_wallpaper(
    renderer: &mut GlesRenderer,
    path: &str,
) -> (TextureBuffer<GlesTexture>, (i32, i32)) {
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();

    let (width, height) = img.dimensions();

    let texture = renderer
        .import_memory(
            img.as_raw(),
            Fourcc::Abgr8888,
            (img.width() as i32, img.height() as i32).into(),
            false,
        )
        .unwrap();
    let buffer = TextureBuffer::from_texture(
        renderer,
        texture,
        1,
        smithay::utils::Transform::Normal,
        None,
    );

    (buffer, (width as i32, height as i32))
}

impl Aurora {
    pub fn ensure_wallpaper_loaded(&mut self, renderer: &mut GlesRenderer) {
        if self.wallpaper.is_some() {
            return;
        }

        let wallpaper_name = "aperture-vintage-NrAvSjyW3D4-unsplash.jpg";

        let (texture, size) = load_wallpaper(
            renderer,
            format!("/run/media/mahantys/Common/Wallpapers/{}", wallpaper_name).as_str(),
        );

        self.wallpaper = Some(texture);
        self.wallpaper_size = Some(size);
    }
}
