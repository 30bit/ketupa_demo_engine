#![allow(dead_code)]
mod graphics;
mod input;
mod layers;
mod screen;
mod tesselator;

use {
    graphics::Graphics,
    pollster::FutureExt as _,
    std::{
        mem::replace,
        time::{Duration, Instant},
    },
    winit::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

pub use {
    glam::{const_mat2, const_vec2, mat2, vec2, Affine2, Mat2, Vec2, Vec2Swizzles},
    input::{Keys, Mouse},
    layers::{
        color, instance, layer_bounds, transform, Color, Instance, Layer, LayerBounds, LayerMut,
        Layers,
    },
    screen::Screen,
    tesselator::{tessellation_chain, TessellationChain, Tessellator},
};

pub struct Setup<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
    pub layers_bounds: &'a [LayerBounds],
}

pub fn setup<'a>(
    title: &'a str,
    width: u32,
    height: u32,
    chunk_bounds: &'a [LayerBounds],
) -> Setup<'a> {
    Setup::new(title, width, height, chunk_bounds)
}

pub struct State<'a> {
    pub layers: &'a mut Layers,
    pub tessellator: &'a mut Tessellator,
    pub screen: &'a mut Screen,
    pub mouse: &'a Mouse,
    pub keys: &'a Keys,
    pub delta: &'a Duration,
}

impl<'a> Setup<'a> {
    pub fn new(title: &'a str, width: u32, height: u32, layers_bounds: &'a [LayerBounds]) -> Self {
        Self {
            title,
            width,
            height,
            layers_bounds,
        }
    }

    #[allow(unused_assignments)]
    pub fn run(self, mut f: impl FnMut(State<'_>) + 'static) -> ! {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = if self.width != 0 && self.height != 0 {
            WindowBuilder::new().with_inner_size(PhysicalSize::new(self.width, self.height))
        } else {
            WindowBuilder::new()
        }
        .with_visible(false)
        .with_title(self.title)
        .build(&event_loop)
        .unwrap();
        let mut frame = Instant::now();
        let mut delta = Duration::ZERO;
        let mut size = window.inner_size();
        let mut screen = Screen::new(size);
        let mut mouse = Mouse::new();
        let mut keys = Keys::new();
        let mut graphics = Graphics::new(&window, Layers::new(self.layers_bounds)).block_on();
        let mut tessellator = Tessellator::with_capacity_to_fit(&graphics.layers);
        window.set_visible(true);
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                if let Some(new_size) = screen.try_process(&event) {
                    size = new_size;
                    window.request_redraw();
                } else if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                } else if !mouse.try_process(&event, screen.half()) {
                    keys.try_process(&event);
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = Instant::now();
                delta = now - replace(&mut frame, now);
                f(State {
                    layers: &mut graphics.layers,
                    tessellator: &mut tessellator,
                    screen: &mut screen,
                    mouse: &mouse,
                    keys: &keys,
                    delta: &delta,
                });
                if graphics
                    .render(size, screen.zoom(), screen.clear_color())
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
                mouse.unset();
                keys.unset();
                screen.unset();
            }
            _ => {}
        })
    }
}
