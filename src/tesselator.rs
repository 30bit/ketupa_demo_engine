use {
    crate::layers::Layers,
    glam::Vec2,
    lyon_tessellation::{
        path::{
            builder::WithSvg,
            path::{Builder as PathBuilder, Path},
            PathEvent,
        },
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, LineCap, LineJoin, StrokeOptions,
        StrokeTessellator, StrokeVertex, VertexBuffers,
    },
    std::mem::replace,
};

pub struct TessellationChain(WithSvg<PathBuilder>);

impl Default for TessellationChain {
    fn default() -> Self {
        Self(PathBuilder::new().with_svg())
    }
}

impl TessellationChain {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(PathBuilder::with_capacity(capacity, capacity).with_svg())
    }

    pub fn chain(&mut self, iter: impl IntoIterator<Item = Vec2>) -> &mut Self {
        let mut points = iter.into_iter();
        self.0.reserve(points.size_hint().0, 0);
        if let Some(first) = points.next() {
            self.0.move_to(mint_convert(first));
        } else {
            return self;
        }

        for to in points {
            self.0.line_to(mint_convert(to));
        }
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.0.close();
        self
    }

    pub fn finish(&mut self) -> Path {
        replace(&mut self.0, PathBuilder::default().with_svg()).build()
    }
}

pub fn tessellation_chain(capacity: usize) -> TessellationChain {
    TessellationChain::with_capacity(capacity)
}

macro_rules! tessellate {
    ($tessellator:ident<$vertex:ident<$($l:lifetime),+>>,$self:ident,$path:expr,$options:expr) => {{
        $self.0.vertices.clear();
        $self.0.indices.clear();
        let mut buffers_builder = BuffersBuilder::new(&mut $self.0,
            |p: $vertex<$($l),+>| mint_convert(p.position())
        );
        let mut t = $tessellator::new();
        let _ = t.tessellate(
            $path,
            &$options,
            &mut buffers_builder
        );
    }};
}

pub struct Tessellator(VertexBuffers<Vec2, u16>);

impl Tessellator {
    pub fn vertices(&self) -> &[Vec2] {
        &self.0.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.0.indices
    }

    pub fn fill_clear(&mut self, path: impl IntoIterator<Item = PathEvent>) {
        self.fill_clear_with(path, FillOptions::default().with_tolerance(0.0001))
    }

    pub fn stroke_clear(&mut self, path: impl IntoIterator<Item = PathEvent>, width: f32) {
        self.stroke_clear_with(
            path,
            StrokeOptions::default()
                .with_tolerance(0.0001)
                .with_line_join(LineJoin::Bevel)
                .with_line_cap(LineCap::Butt)
                .with_line_width(width),
        )
    }

    pub fn fill_clear_with(
        &mut self,
        path: impl IntoIterator<Item = PathEvent>,
        options: FillOptions,
    ) {
        tessellate!(FillTessellator<FillVertex<'_>>, self, path, options);
    }

    pub fn stroke_clear_with(
        &mut self,
        path: impl IntoIterator<Item = PathEvent>,
        options: StrokeOptions,
    ) {
        tessellate!(StrokeTessellator<StrokeVertex<'_, '_>>, self, path, options);
    }

    pub(crate) fn with_capacity_to_fit(chunk: &Layers) -> Self {
        if chunk.ranges.is_empty() {
            return Self(VertexBuffers::new());
        }
        let mut vertex_capacity = 0;
        let mut index_capacity = 0;
        let mut prev = unsafe { chunk.ranges.get_unchecked(0) };
        for i in 1..chunk.ranges.len() {
            unsafe {
                let next = chunk.ranges.get_unchecked(i);
                vertex_capacity = vertex_capacity.max(next.vertex_start - prev.vertex_start);
                index_capacity = index_capacity.max(next.index_start - prev.index_start);
                prev = next;
            }
        }
        vertex_capacity = vertex_capacity.max(chunk.vertices.len() as u16 - prev.vertex_start);
        index_capacity = index_capacity.max(chunk.indices.len() as u32 - prev.index_start);
        Self(VertexBuffers::with_capacity(
            vertex_capacity as _,
            index_capacity as _,
        ))
    }
}

fn mint_convert<P: From<mint::Point2<f32>>>(p: impl Into<mint::Point2<f32>>) -> P {
    p.into().into()
}
