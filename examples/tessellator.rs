use ketupa_demo_engine::{color, instance, layer_bounds, setup, tessellation_chain, vec2};

fn main() {
    setup("Hello World", 1000, 1000, &[layer_bounds(16, 16, 1)]).run(|st| {
        st.screen.set_clear_color(color(235, 64, 52, 255));
        st.tessellator.fill_clear(
            tessellation_chain(5)
                .chain([
                    vec2(-120.0, 25.0),
                    vec2(-45.0, 185.0),
                    vec2(80.0, 135.0),
                    vec2(105.0, -90.0),
                    vec2(-20.0, -265.0),
                ])
                .finish()
                .iter(),
        );
        let mut layer = st.layers.get_mut(0).unwrap();
        layer.set_vertices(st.tessellator.vertices().iter().cloned());
        layer.set_indices(st.tessellator.indices().iter().cloned());
        layer.set_instances([instance(Default::default(), color(252, 186, 3, 255))])
    })
}
