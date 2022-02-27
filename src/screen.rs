use {
    crate::layers::Color,
    glam::{vec2, Vec2},
    winit::{dpi::PhysicalSize, event::WindowEvent},
};

pub struct Screen {
    size: Vec2,
    half: Vec2,
    zoom: f32,
    zoom_recip: f32,
    has_resized: bool,
    clear_color: Color,
}

impl Screen {
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.zoom_recip = zoom.recip();
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn half(&self) -> Vec2 {
        self.half
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn zoom_recip(&self) -> f32 {
        self.zoom_recip
    }

    pub fn world_of(&self, local: Vec2) -> Vec2 {
        local * self.zoom
    }

    pub fn local_of(&self, world: Vec2) -> Vec2 {
        world * self.zoom_recip
    }

    pub fn has_resized(&self) -> bool {
        self.has_resized
    }

    pub fn clear_color(&self) -> Color {
        self.clear_color
    }

    pub(crate) fn new(size: PhysicalSize<u32>) -> Self {
        let size = vec2(size.width as _, size.height as _);
        Self {
            size,
            half: size * 0.5,
            zoom: 1.0,
            zoom_recip: 1.0,
            has_resized: true,
            clear_color: Color::new(0, 0, 0, 0),
        }
    }

    pub(crate) fn try_process(&mut self, event: &WindowEvent) -> Option<PhysicalSize<u32>> {
        if let WindowEvent::Resized(size)
        | WindowEvent::ScaleFactorChanged {
            new_inner_size: &mut size,
            ..
        } = *event
        {
            self.size = vec2(size.width as _, size.height as _);
            self.half = self.size * 0.5;
            self.has_resized = true;
            Some(size)
        } else {
            None
        }
    }

    pub(crate) fn unset(&mut self) {
        self.has_resized = false
    }
}
