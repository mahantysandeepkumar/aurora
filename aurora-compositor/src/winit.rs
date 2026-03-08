use std::time::Duration;

use smithay::{
    backend::{
        renderer::{
            damage::OutputDamageTracker,
            element::{
                AsRenderElements, Kind, texture::TextureRenderElement, utils::RescaleRenderElement,
            },
            gles::{GlesRenderer, GlesTexture},
        },
        winit::{self, WinitEvent},
    },
    desktop::{
        Window,
        space::{SpaceRenderElements, space_render_elements},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::EventLoop,
    utils::{Point, Rectangle, Scale, Transform},
};

use crate::state::Aurora;
smithay::render_elements! {
    pub CustomRenderElements<=GlesRenderer>;

    // Ask the AsRenderElements trait what type the Window uses to render
    Space=SpaceRenderElements<GlesRenderer, <Window as AsRenderElements<GlesRenderer>>::RenderElement>,

    Wallpaper=RescaleRenderElement<TextureRenderElement<GlesTexture>>,
}

pub fn init_winit(
    event_loop: &mut EventLoop<Aurora>,
    state: &mut Aurora,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut backend, winit_loop) = winit::init()?;
    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000, // fixed to 60hz for now
    };

    println!("Wayland display created.");
    let output = Output::new(
        "winit".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "Winit".into(),
            serial_number: "Unknown".into(),
        },
    );

    let _global = output.create_global::<Aurora>(&state.display_handle);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180), // why flipped 180 ???
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);
    state.space.map_output(&output, (0, 0));

    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    event_loop
        .handle()
        .insert_source(winit_loop, move |event, _, state| {
            match event {
                WinitEvent::Resized { size, .. } => {
                    output.change_current_state(
                        Some(Mode {
                            size,            // change size to new resized
                            refresh: 60_000, // keep refreshrate same
                        }),
                        None,
                        None,
                        None,
                    ); // no need to change sclaing, transform etc
                }
                WinitEvent::Input(event) => state.process_input_event(event), //not implemented yet
                WinitEvent::Redraw => {
                    // get the window size
                    let size = backend.window_size();
                    // get the changed area
                    let damage = Rectangle::from_size(size);
                    {
                        let (renderer, mut frame_buffer) = backend.bind().unwrap();

                        state.ensure_wallpaper_loaded(renderer);

                        let mut render_elements: Vec<CustomRenderElements> = Vec::new();

                        // First add all the windows then the wallpaper else the wallapaper will damage everything  else
                        let space_elements = space_render_elements(
                            renderer,
                            [&state.space],
                            &output,
                            1.0, // alpha
                        )
                        .expect("Failed to generate space render elements");

                        for element in space_elements {
                            render_elements.push(CustomRenderElements::Space(element));
                        }

                        if let Some(wallpaper) = &state.wallpaper {
                            let wallpaper_element = TextureRenderElement::from_texture_buffer(
                                (0.0, 0.0), // location
                                wallpaper,  // buffer
                                None,       // alpha override
                                None,
                                None,              // source crop
                                Kind::Unspecified, // damage tracking kind
                            );

                            // Calculate the scale multiplier
                            let bg_size = state.wallpaper_size.unwrap();
                            let scale_x = size.w as f64 / bg_size.0 as f64;
                            let scale_y = size.h as f64 / bg_size.1 as f64;
                            let scale = Scale::from((scale_x, scale_y));

                            // Scale the texture
                            let scaled_element = RescaleRenderElement::from_element(
                                wallpaper_element,
                                Point::from((0, 0)), // Origin point
                                scale,               // The calculated stretch factor
                            );

                            render_elements.push(CustomRenderElements::Wallpaper(scaled_element));
                        }

                        // 4. Render the perfectly ordered stack!
                        let clear_color = [0.0, 0.0, 0.0, 1.0];

                        damage_tracker
                            .render_output(
                                renderer,
                                &mut frame_buffer,
                                0, // age
                                &render_elements,
                                clear_color,
                            )
                            .expect("Failed to render output");
                    }
                    backend.submit(Some(&[damage])).unwrap();

                    // do some space rerlated stuff later and clear them
                    state.space.elements().for_each(|window| {
                        window.send_frame(
                            &output,
                            state.start_time.elapsed(),
                            Some(Duration::ZERO),
                            |_, _| Some(output.clone()),
                        )
                    });

                    state.space.refresh();
                    state.popups.cleanup();

                    let _ = state.display_handle.flush_clients();

                    // Ask for redraw to schedule new frame.
                    backend.window().request_redraw();
                }
                WinitEvent::CloseRequested => {
                    state.loop_signal.stop();
                }
                _ => (),
            };
        })?;
    Ok(())
}
