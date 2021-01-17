use crate::{Application, RenderRegion, MouseMoveEvent, MouseLeaveEvent, MouseEnterEvent, GolemRenderer};

use golem::*;

use glutin::{
    dpi::PhysicalPosition, dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, ContextWrapper, PossiblyCurrent, window::Window
};

use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use golem::Dimension::D2;

pub fn start(mut app: Application, title: &str) {
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_maximized(false)
        .with_resizable(true)
        .with_title(title)
        .with_visible(true);
    let windowed_context = unsafe {
        glutin::ContextBuilder::new()
            .build_windowed(builder, &event_loop)
            .expect("Should be able to create a window")
            .make_current()
            .expect("Should be able to make context current")
    };

    let golem = Context::from_glow(glow::Context::from_loader_function(|function_name| {
        windowed_context.get_proc_address(function_name)
    }))
    .expect("Should be able to create Golem context");
    let renderer = GolemRenderer::new(golem);

    let mut start_time = Instant::now();

    let mut mouse_position: Option<PhysicalPosition<i32>> = None;
    let mut should_fire_mouse_enter_event = false;

    let mut render_surface: Option<Surface> = None;

    event_loop.run(move |event, _target, control_flow| {
        // I use `Poll` instead of `Wait` to get more control over the control flow.
        // I use a simple custom system to avoid too large power usage
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => {
                match window_event {
                    WindowEvent::Resized(_) => {
                        // TODO app.on_resize
                        println!("Resize");
                        render_surface = None;
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        ..
                    } => {
                        if state == ElementState::Released {
                            // It would be weird if we don't have a mouse position
                            if let Some(click_position) = mouse_position {
                                // Just 1 mouse on desktops
                                let knukki_mouse = crate::Mouse::new(0);

                                // Convert winit mouse position to knukki mouse position
                                let window_size = windowed_context.window().inner_size();
                                let knukki_x = click_position.x as f32 / window_size.width as f32;
                                let knukki_y =
                                    1.0 - (click_position.y as f32 / window_size.height as f32);
                                let knukki_point = crate::Point::new(knukki_x, knukki_y);

                                // Convert winit button to knukki button
                                let knukki_button = match button {
                                    MouseButton::Left => crate::MouseButton::primary(),
                                    MouseButton::Right => crate::MouseButton::new(1),
                                    MouseButton::Middle => crate::MouseButton::new(2),
                                    MouseButton::Other(id) => crate::MouseButton::new(id),
                                };

                                // Construct and fire the event
                                let knukki_event = crate::MouseClickEvent::new(
                                    knukki_mouse,
                                    knukki_point,
                                    knukki_button,
                                );

                                app.fire_mouse_click_event(knukki_event);
                            }
                        }
                    }
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        ..
                    } => {
                        // Winit seems to fire mouse move events in occasions like clicking on the
                        // app icon in the taskbar or opening the window, even when the cursor is
                        // not inside the window. Let's just ignore these events.
                        let window_size = windowed_context.window().inner_size();
                        if position.x < 0 || position.y < 0 ||
                            (position.x as u32) >= window_size.width ||
                            (position.y as u32) >= window_size.height {
                            return;
                        }
                        // If there is a previous mouse position, fire a move event
                        if let Some(previous_position) = mouse_position {
                            // Winit seems to fire a double cursor move event when the cursor enters
                            // the window. I don't know if this happens more often, but let's be
                            // careful and not propagate move events between equal positions.
                            if previous_position.x != position.x || previous_position.y != position.y {
                                let old_x = previous_position.x as f32 / window_size.width as f32;
                                let old_y = 1.0 - previous_position.y as f32 / window_size.height as f32;
                                let new_x = position.x as f32 / window_size.width as f32;
                                let new_y = 1.0 - position.y as f32 / window_size.height as f32;
                                let event = MouseMoveEvent::new(
                                    crate::Mouse::new(0),
                                    crate::Point::new(old_x, old_y),
                                    crate::Point::new(new_x, new_y)
                                );
                                app.fire_mouse_move_event(event);
                            }
                        } else {
                            if should_fire_mouse_enter_event {
                                let x = position.x as f32 / window_size.width as f32;
                                let y = 1.0 - position.y as f32 / window_size.height as f32;
                                let event = MouseEnterEvent::new(
                                    crate::Mouse::new(0), crate::Point::new(x, y)
                                );
                                app.fire_mouse_enter_event(event);
                                should_fire_mouse_enter_event = false;
                            }
                        }
                        mouse_position = Some(position);
                    }
                    WindowEvent::CursorEntered { .. } => {
                        should_fire_mouse_enter_event = true;
                    },
                    WindowEvent::CursorLeft { .. } => {
                        // If we know where the cursor was, we should fire a MouseLeaveEvent
                        if let Some(previous_position) = mouse_position {
                            let window_size = windowed_context.window().inner_size();
                            let old_x = previous_position.x as f32 / window_size.width as f32;
                            let old_y = 1.0 - previous_position.y as f32 / window_size.height as f32;
                            let event = MouseLeaveEvent::new(
                                crate::Mouse::new(0), crate::Point::new(old_x, old_y)
                            );
                            app.fire_mouse_leave_event(event);
                        }

                        // Once the mouse leaves the window, we have no clue where it is, but it
                        // won't be at this mouse position
                        mouse_position = None;
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                // Let the application decide whether it needs to redraw itself
                let force = false;

                // Draw onto the entire inner window buffer
                let size = windowed_context.window().inner_size();

                // Give the application a render opportunity every ~16 milliseconds
                let current_time = Instant::now();
                let elapsed_time = (current_time - start_time).as_millis();
                if elapsed_time < 16 {
                    sleep(Duration::from_millis(16 - elapsed_time as u64));
                }
                start_time = Instant::now();

                draw_application(&mut app, &renderer, &mut render_surface, size, force, &windowed_context).expect("Should be able to draw app");
            }
            Event::RedrawRequested(_) => {
                // This provider will never request a winit redraw, so when this
                // event is fired, it must have come from the OS.
                let force = true;

                // Draw onto the entire inner window buffer
                let size = windowed_context.window().inner_size();

                draw_application(&mut app, &renderer, &mut render_surface, size, force, &windowed_context).expect("Should be able to force draw app");
            }
            _ => (),
        }
    });

    fn draw_application(
        app: &mut Application, renderer: &GolemRenderer,
        render_surface: &mut Option<Surface>,
        size: PhysicalSize<u32>, force: bool, windowed_context: &ContextWrapper<PossiblyCurrent, Window>
    ) -> Result<(), GolemError> {
        let region = RenderRegion::with_size(0, 0, size.width, size.height);
        let golem = renderer.get_context();

        let mut created_surface = false;

        // Make sure there is an up-to-date render texture to draw the application on
        if render_surface.is_none() {
            let mut render_texture = Texture::new(golem).expect("Should be able to create texture");
            render_texture.set_image(None, size.width, size.height, ColorFormat::RGBA);
            *render_surface = Some(Surface::new(golem, render_texture).expect("Should be able to create surface"));
            created_surface = true;
        }

        // Draw the application on the render texture
        let render_surface = render_surface.as_ref().unwrap();
        if !render_surface.is_bound() {
            render_surface.bind();
            println!("Bind render surface");
        }
        if app.render(renderer, region, force || created_surface) {
            println!("Expected pixels:");
            let mut pixel_data = vec![0; 4 * size.width as usize * size.height as usize];
            render_surface.get_pixel_data(0, 0, size.width, size.height, ColorFormat::RGBA, &mut pixel_data);
            let mut y = 0;
            while y < size.height {
                let mut x = 0;
                while x < size.width {
                    let index = 4 * (x + y * size.width) as usize;
                    print!("{}{}{} ", pixel_data[index + 0] / 26, pixel_data[index + 1] / 26, pixel_data[index + 2] / 26);
                    x += 30;
                }
                y += 30;
                println!();
            }
            println!();
            println!();

            // Draw the render texture onto the presenting texture
            println!("Unbind render surface");
            Surface::unbind(renderer.get_context());
            golem.set_viewport(0, 0, size.width, size.height);
            golem.disable_scissor();
            // TODO Improve performance by creating the GPU resources only once
            let mut vb = VertexBuffer::new(golem)?;
            let mut eb = ElementBuffer::new(golem)?;

            #[rustfmt::skip]
                let vertices = [
                -1.0, -1.0,
                1.0, -1.0,
                1.0, 1.0,
                -1.0, 1.0,
            ];
            let indices = [0, 1, 2, 2, 3, 0];
            let mut shader = ShaderProgram::new(
                golem,
                ShaderDescription {
                    vertex_input: &[
                        Attribute::new("position", AttributeType::Vector(D2)),
                    ],
                    fragment_input: &[Attribute::new("passPosition", AttributeType::Vector(D2))],
                    uniforms: &[
                        Uniform::new("image", UniformType::Sampler2D),
                    ],
                    vertex_shader: r#" void main() {
            gl_Position = vec4(position.x, position.y, 0.0, 1.0);
            passPosition = position;
        }"#,
                    fragment_shader: r#" void main() {
            vec4 theColor = texture(image, vec2(0.5 + passPosition.x * 0.5, 0.5 + passPosition.y * 0.5));
            gl_FragColor = theColor;
        }"#,
                },
            )?;
            vb.set_data(&vertices);
            eb.set_data(&indices);
            shader.bind();
            shader.prepare_draw(&vb, &eb)?;
            shader.set_uniform("image", UniformValue::Int(1))?;

            let bind_point = std::num::NonZeroU32::new(1).unwrap();
            unsafe {
                let texture = render_surface.borrow_texture().unwrap();
                texture.set_active(bind_point);
            }
            unsafe {
                shader.draw_prepared(0..indices.len(), GeometryMode::Triangles);
            }

            windowed_context.swap_buffers().expect("Good context");
        }
        Ok(())
    }
}
