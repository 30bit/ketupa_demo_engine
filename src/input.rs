use {
    glam::{vec2, Vec2},
    std::mem::{replace, take},
    winit::event::ElementState,
};

pub use winit::event::{
    ModifiersState, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent,
};

pub struct Mouse {
    pub position: Vec2,
    pub velocity: Vec2,
    pub scroll: Vec2,
    pub has_entered: bool,
    pub has_left: bool,
    pub has_moved: bool,
    pub has_scrolled: bool,
    map: Map<3>,
}

macro_rules! map_mouse_button {
    ($self:ident::$f:ident($button:expr)$(, $default:expr)?) => {
        match $button {
            MouseButton::Left => $self.map.$f(0),
            MouseButton::Middle => $self.map.$f(1),
            MouseButton::Right => $self.map.$f(2),
            _ => {$($default)?},
        }
    };
}

impl Mouse {
    pub const LEFT: MouseButton = MouseButton::Left;

    pub const RIGHT: MouseButton = MouseButton::Right;

    pub const MIDDLE: MouseButton = MouseButton::Middle;

    pub fn is_just_pressed(&self, button: MouseButton) -> bool {
        map_mouse_button!(self::is_just_pressed(button), false)
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        map_mouse_button!(self::is_pressed(button), false)
    }

    pub fn is_released(&self, button: MouseButton) -> bool {
        map_mouse_button!(self::is_released(button), false)
    }

    pub(crate) fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            scroll: Vec2::ZERO,
            has_entered: false,
            has_left: false,
            has_moved: false,
            has_scrolled: false,
            map: Map::new(),
        }
    }

    pub(crate) fn try_process(&mut self, event: &WindowEvent, screen_half: Vec2) -> bool {
        match event {
            WindowEvent::CursorEntered { .. } => self.has_entered = true,
            WindowEvent::CursorLeft { .. } => self.has_left = true,
            WindowEvent::CursorMoved { position, .. } => {
                self.has_moved = true;
                self.velocity = replace(
                    &mut self.position,
                    vec2(
                        position.x as f32 - screen_half.x,
                        screen_half.y - position.y as f32,
                    ),
                );
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *state == ElementState::Pressed {
                    map_mouse_button!(self::press(button));
                } else {
                    map_mouse_button!(self::release(button));
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.has_scrolled = true;
                self.scroll = match delta {
                    MouseScrollDelta::LineDelta(x, y) => vec2(*x, *y),
                    MouseScrollDelta::PixelDelta(xy) => vec2(xy.x as _, -(xy.y as f32)),
                };
            }
            _ => return false,
        };
        true
    }

    pub(crate) fn unset(&mut self) {
        self.map.unset();
        self.has_entered = false;
        self.has_left = false;
        if !take(&mut self.has_moved) {
            self.velocity = Vec2::ZERO;
        }
        if !take(&mut self.has_scrolled) {
            self.scroll = Vec2::ZERO;
        }
    }
}

pub struct Keys {
    pub is_shift: bool,
    pub is_ctrl: bool,
    pub is_alt: bool,
    pub is_logo: bool,
    map: Map<163>,
}

macro_rules! map_keyboard_key {
    ($self:ident::$f:ident($key:expr)) => {{
        let index = $key as usize;
        $self.map.$f(index)
    }};
}

impl Keys {
    pub const LEFT: VirtualKeyCode = VirtualKeyCode::Left;
    pub const RIGHT: VirtualKeyCode = VirtualKeyCode::Right;
    pub const UP: VirtualKeyCode = VirtualKeyCode::Up;
    pub const DOWN: VirtualKeyCode = VirtualKeyCode::Down;
    pub const A: VirtualKeyCode = VirtualKeyCode::A;
    pub const B: VirtualKeyCode = VirtualKeyCode::B;
    pub const C: VirtualKeyCode = VirtualKeyCode::C;
    pub const D: VirtualKeyCode = VirtualKeyCode::D;
    pub const E: VirtualKeyCode = VirtualKeyCode::E;
    pub const F: VirtualKeyCode = VirtualKeyCode::F;
    pub const G: VirtualKeyCode = VirtualKeyCode::G;
    pub const H: VirtualKeyCode = VirtualKeyCode::H;
    pub const I: VirtualKeyCode = VirtualKeyCode::I;
    pub const J: VirtualKeyCode = VirtualKeyCode::J;
    pub const K: VirtualKeyCode = VirtualKeyCode::K;
    pub const L: VirtualKeyCode = VirtualKeyCode::L;
    pub const M: VirtualKeyCode = VirtualKeyCode::M;
    pub const N: VirtualKeyCode = VirtualKeyCode::N;
    pub const O: VirtualKeyCode = VirtualKeyCode::O;
    pub const P: VirtualKeyCode = VirtualKeyCode::P;
    pub const Q: VirtualKeyCode = VirtualKeyCode::Q;
    pub const R: VirtualKeyCode = VirtualKeyCode::R;
    pub const S: VirtualKeyCode = VirtualKeyCode::S;
    pub const T: VirtualKeyCode = VirtualKeyCode::T;
    pub const U: VirtualKeyCode = VirtualKeyCode::U;
    pub const V: VirtualKeyCode = VirtualKeyCode::V;
    pub const W: VirtualKeyCode = VirtualKeyCode::W;
    pub const X: VirtualKeyCode = VirtualKeyCode::X;
    pub const Y: VirtualKeyCode = VirtualKeyCode::Y;
    pub const Z: VirtualKeyCode = VirtualKeyCode::Z;
    pub const NUM_0: VirtualKeyCode = VirtualKeyCode::Key0;
    pub const NUM_1: VirtualKeyCode = VirtualKeyCode::Key1;
    pub const NUM_2: VirtualKeyCode = VirtualKeyCode::Key2;
    pub const NUM_3: VirtualKeyCode = VirtualKeyCode::Key3;
    pub const NUM_4: VirtualKeyCode = VirtualKeyCode::Key4;
    pub const NUM_5: VirtualKeyCode = VirtualKeyCode::Key5;
    pub const NUM_6: VirtualKeyCode = VirtualKeyCode::Key6;
    pub const NUM_7: VirtualKeyCode = VirtualKeyCode::Key7;
    pub const NUM_8: VirtualKeyCode = VirtualKeyCode::Key8;
    pub const NUM_9: VirtualKeyCode = VirtualKeyCode::Key9;

    pub fn is_just_pressed(&self, key: VirtualKeyCode) -> bool {
        map_keyboard_key!(self::is_just_pressed(key))
    }

    pub fn is_pressed(&self, key: VirtualKeyCode) -> bool {
        map_keyboard_key!(self::is_pressed(key))
    }

    pub fn is_released(&self, key: VirtualKeyCode) -> bool {
        map_keyboard_key!(self::is_released(key))
    }

    pub(crate) fn new() -> Self {
        Self {
            is_shift: false,
            is_ctrl: false,
            is_alt: false,
            is_logo: false,
            map: Map::new(),
        }
    }

    pub(crate) fn try_process(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } if input.virtual_keycode.is_some() => {
                let keycode = unsafe { input.virtual_keycode.unwrap_unchecked() };
                if input.state == ElementState::Pressed {
                    map_keyboard_key!(self::press(keycode))
                } else {
                    map_keyboard_key!(self::release(keycode))
                }
            }
            WindowEvent::ModifiersChanged(state) => {
                self.is_shift = state.shift();
                self.is_ctrl = state.ctrl();
                self.is_alt = state.alt();
                self.is_logo = state.logo();
            }
            _ => return false,
        }
        true
    }

    pub(crate) fn unset(&mut self) {
        self.map.unset();
    }
}

struct Map<const N: usize> {
    not_just_pressed: [bool; N],
    pressed: [bool; N],
    released: [bool; N],
}

impl<const N: usize> Map<N> {
    fn new() -> Self {
        Self {
            not_just_pressed: [true; N],
            pressed: [false; N],
            released: [false; N],
        }
    }

    fn press(&mut self, index: usize) {
        unsafe { *self.pressed.get_unchecked_mut(index) = true }
    }

    fn release(&mut self, index: usize) {
        unsafe { *self.released.get_unchecked_mut(index) = true }
    }

    fn unset(&mut self) {
        for i in 0..N {
            self.not_just_pressed[i] = self.pressed[i];
            self.pressed[i] &= !take(&mut self.released[i]);
        }
    }

    fn is_just_pressed(&self, index: usize) -> bool {
        unsafe { self.pressed.get_unchecked(index) & !self.not_just_pressed.get_unchecked(index) }
    }

    fn is_pressed(&self, index: usize) -> bool {
        unsafe { *self.pressed.get_unchecked(index) }
    }

    fn is_released(&self, index: usize) -> bool {
        unsafe { *self.released.get_unchecked(index) }
    }
}
