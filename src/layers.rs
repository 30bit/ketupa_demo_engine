use {
    bytemuck::{Pod, Zeroable},
    glam::{Affine2, Vec2},
};

pub fn transform(scale: Vec2, angle: f32, translation: Vec2) -> Affine2 {
    Affine2::from_scale_angle_translation(scale, angle, translation)
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn with_r(self, r: u8) -> Self {
        Self { r, ..self }
    }

    pub fn with_g(self, g: u8) -> Self {
        Self { g, ..self }
    }

    pub fn with_b(self, b: u8) -> Self {
        Self { b, ..self }
    }

    pub fn with_a(self, a: u8) -> Self {
        Self { a, ..self }
    }
}

pub const fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Default)]
pub struct Instance {
    pub transform: [f32; 6],
    pub color: Color,
}

unsafe impl Pod for Instance {}
unsafe impl Zeroable for Instance {}

impl Instance {
    pub fn new(transform: Affine2, color: Color) -> Self {
        Self {
            transform: transform.to_cols_array(),
            color,
        }
    }

    pub fn with_transform(self, transform: Affine2) -> Self {
        Self {
            transform: transform.to_cols_array(),
            ..self
        }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

pub fn instance(transform: Affine2, color: Color) -> Instance {
    Instance::new(transform, color)
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct LayerBounds {
    pub max_vertices: u16,
    pub max_indices: u32,
    pub max_instances: u32,
}

impl LayerBounds {
    pub const fn new(max_vertices: u16, max_indices: u32, max_instances: u32) -> Self {
        Self {
            max_vertices,
            max_indices,
            max_instances,
        }
    }
}

pub const fn layer_bounds(max_vertices: u16, max_indices: u32, max_instances: u32) -> LayerBounds {
    LayerBounds::new(max_vertices, max_indices, max_instances)
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct Range {
    pub(crate) vertex_start: u16,
    pub(crate) vertex_floating_end: u16,
    pub(crate) index_start: u32,
    pub(crate) instance_start: u32,
    pub(crate) index_floating_end: u32,
    pub(crate) instance_floating_end: u32,
}

impl Range {
    pub(crate) fn vertex_range(&self) -> std::ops::Range<usize> {
        self.vertex_start as _..self.vertex_floating_end as _
    }

    pub(crate) fn instance_range(&self) -> std::ops::Range<usize> {
        self.instance_start as _..self.instance_floating_end as _
    }

    pub(crate) fn index_range32(&self) -> std::ops::Range<u32> {
        self.index_start..self.index_floating_end
    }

    pub(crate) fn instance_range32(&self) -> std::ops::Range<u32> {
        self.instance_start..self.instance_floating_end
    }
}

pub struct Layer<'a> {
    vertices: &'a [Vec2],
    instances: &'a [Instance],
    indices: &'a [u16],
    range: &'a Range,
    vertex_end: u16,
    index_end: u32,
    instance_end: u32,
}

impl<'a> Layer<'a> {
    pub fn vertices_len(&self) -> usize {
        (self.range.vertex_floating_end - self.range.vertex_start) as _
    }

    pub fn indices_len(&self) -> usize {
        (self.range.index_floating_end - self.range.index_start) as _
    }

    pub fn instances_len(&self) -> usize {
        (self.range.instance_floating_end - self.range.instance_start) as _
    }

    pub fn max_vertices_len(&self) -> usize {
        (self.vertex_end - self.range.vertex_start) as _
    }

    pub fn max_indices_len(&self) -> usize {
        (self.index_end - self.range.index_start) as _
    }

    pub fn max_instances_len(&self) -> usize {
        (self.instance_end - self.range.instance_start) as _
    }

    pub fn vertices(&self) -> &[Vec2] {
        unsafe { self.vertices.get_unchecked(self.range.vertex_range()) }
    }

    pub fn get_index(&self, at: usize) -> Option<u16> {
        self.indices
            .get(at + self.range.index_start as usize)
            .map(|i| i - self.range.vertex_start)
    }

    pub fn instances(&self) -> &[Instance] {
        unsafe { self.instances.get_unchecked(self.range.instance_range()) }
    }
}

pub struct LayerMut<'a> {
    vertices: &'a mut [Vec2],
    instances: &'a mut [Instance],
    indices: &'a mut [u16],
    range: &'a mut Range,
    vertex_end: u16,
    index_end: u32,
    instance_end: u32,
}

impl<'a> LayerMut<'a> {
    pub fn vertices_len(&self) -> usize {
        (self.range.vertex_floating_end - self.range.vertex_start) as _
    }

    pub fn indices_len(&self) -> usize {
        (self.range.index_floating_end - self.range.index_start) as _
    }

    pub fn instances_len(&self) -> usize {
        (self.range.instance_floating_end - self.range.instance_start) as _
    }

    pub fn max_vertices_len(&self) -> usize {
        (self.vertex_end - self.range.vertex_start) as _
    }

    pub fn max_indices_len(&self) -> usize {
        (self.index_end - self.range.index_start) as _
    }

    pub fn max_instances_len(&self) -> usize {
        (self.instance_end - self.range.instance_start) as _
    }

    pub fn vertices(&self) -> &[Vec2] {
        unsafe { self.vertices.get_unchecked(self.range.vertex_range()) }
    }

    pub fn get_index(&self, at: usize) -> Option<u16> {
        self.indices
            .get(at + self.range.index_start as usize)
            .map(|i| i - self.range.vertex_start)
    }

    pub fn instances(&self) -> &[Instance] {
        unsafe { self.instances.get_unchecked(self.range.instance_range()) }
    }

    pub fn vertices_mut(&mut self) -> &mut [Vec2] {
        unsafe { self.vertices.get_unchecked_mut(self.range.vertex_range()) }
    }

    pub fn set_index(&mut self, at: usize, value: u16) -> bool {
        if let Some(dest) = self.indices.get_mut(at) {
            *dest = value + self.range.vertex_start;
            true
        } else {
            false
        }
    }

    pub fn instances_mut(&mut self) -> &mut [Instance] {
        unsafe {
            self.instances
                .get_unchecked_mut(self.range.instance_range())
        }
    }

    pub fn clear_vertices(&mut self) {
        self.range.vertex_floating_end = self.range.vertex_start;
    }

    pub fn clear_indices(&mut self) {
        self.range.index_floating_end = self.range.index_start;
    }

    pub fn clear_instances(&mut self) {
        self.range.instance_floating_end = self.range.instance_start;
    }

    pub fn truncate_vertices(&mut self, len: usize) {
        let offset = self.range.vertex_start + len as u16;
        if offset < self.vertex_end {
            return;
        }
        self.range.vertex_floating_end = offset;
    }

    pub fn truncate_indices(&mut self, len: usize) {
        let offset = self.range.index_start + len as u32;
        if offset < self.index_end {
            return;
        }
        self.range.index_floating_end = offset;
    }

    pub fn truncate_instances(&mut self, len: usize) {
        let offset = self.range.instance_start + len as u32;
        if offset < self.instance_end {
            return;
        }
        self.range.instance_floating_end = offset;
    }

    pub fn extend_vertices(&mut self, iter: impl IntoIterator<Item = Vec2>) {
        let slice = unsafe {
            self.vertices
                .get_unchecked_mut(self.range.vertex_floating_end as _..self.vertex_end as usize)
        };
        let mut additional = iter.into_iter();
        for dest in slice.iter_mut() {
            if let Some(src) = additional.next() {
                *dest = src;
                self.range.vertex_floating_end += 1;
            } else {
                return;
            }
        }
    }

    pub fn extend_indices(&mut self, iter: impl IntoIterator<Item = u16>) {
        let slice = unsafe {
            self.indices
                .get_unchecked_mut(self.range.index_floating_end as _..self.index_end as usize)
        };
        let mut additional = iter.into_iter();
        for dest in slice.iter_mut() {
            if let Some(src) = additional.next() {
                *dest = src + self.range.vertex_start;
                self.range.index_floating_end += 1;
            } else {
                return;
            }
        }
    }

    pub fn extend_instances(&mut self, iter: impl IntoIterator<Item = Instance>) {
        let slice = unsafe {
            self.instances.get_unchecked_mut(
                self.range.instance_floating_end as _..self.instance_end as usize,
            )
        };
        let mut additional = iter.into_iter();
        for dest in slice.iter_mut() {
            if let Some(src) = additional.next() {
                *dest = src;
                self.range.instance_floating_end += 1;
            } else {
                return;
            }
        }
    }

    pub fn set_vertices(&mut self, iter: impl IntoIterator<Item = Vec2>) {
        self.clear_vertices();
        self.extend_vertices(iter);
    }

    pub fn set_indices(&mut self, iter: impl IntoIterator<Item = u16>) {
        self.clear_indices();
        self.extend_indices(iter);
    }

    pub fn set_instances(&mut self, iter: impl IntoIterator<Item = Instance>) {
        self.clear_instances();
        self.extend_instances(iter);
    }
}

pub struct Layers {
    pub(crate) vertices: Box<[Vec2]>,
    pub(crate) indices: Box<[u16]>,
    pub(crate) instances: Box<[Instance]>,
    pub(crate) ranges: Box<[Range]>,
}

macro_rules! get {
    ($name:ident($($mut:ident)? &$self:ident::$get_range:ident($index:expr))) => {{
        let chunk = $self.ranges.len().checked_sub($index + 1)?;
        let (vertex_end, index_end, instance_end) = {
            let next_chunk = chunk + 1;
            if next_chunk == $self.ranges.len() {
                (
                    $self.vertices.len() as _,
                    $self.indices.len() as _,
                    $self.instances.len() as _,
                )
            } else {
                let next_range = unsafe { $self.ranges.get_unchecked(next_chunk) };
                (
                    next_range.vertex_start,
                    next_range.index_start,
                    next_range.instance_start,
                )
            }
        };
        let range = unsafe { $self.ranges.$get_range(chunk) };
        Some($name {
            vertices: &$($mut)? $self.vertices,
            indices: &$($mut)? $self.indices,
            instances: &$($mut)? $self.instances,
            range,
            vertex_end,
            index_end,
            instance_end,
        })
    }};
}

impl Layers {
    pub fn get(&self, index: usize) -> Option<Layer> {
        get!(Layer(&self::get_unchecked(index)))
    }

    pub fn get_mut(&mut self, index: usize) -> Option<LayerMut> {
        get!(LayerMut(mut &self::get_unchecked_mut(index)))
    }

    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub(crate) fn new(bounds: &[LayerBounds]) -> Self {
        let mut ranges = vec![Range::default(); bounds.len()].into_boxed_slice();
        let mut vertex_start = 0;
        let mut index_start = 0;
        let mut instance_start = 0;
        for i in 0..bounds.len() {
            let range = unsafe { ranges.get_unchecked_mut(i) };
            range.vertex_start = vertex_start;
            range.vertex_floating_end = vertex_start;
            range.index_start = index_start;
            range.index_floating_end = index_start;
            range.instance_start = instance_start;
            range.instance_floating_end = instance_start;
            let bound = unsafe { bounds.get_unchecked(bounds.len() - i - 1) };
            vertex_start += bound.max_vertices;
            index_start += bound.max_indices;
            instance_start += bound.max_instances;
        }
        let vertices = vec![Vec2::ZERO; vertex_start as _].into_boxed_slice();
        let indices = vec![0u16; index_start as _].into_boxed_slice();
        let instances = vec![Instance::default(); instance_start as _].into_boxed_slice();
        Self {
            vertices,
            indices,
            instances,
            ranges,
        }
    }
}
