use ketupa_demo_engine::{instance, layer_bounds, setup, vec2, color};

fn main() {
    setup("Hello World", 1000, 1000, &[layer_bounds(8, 8, 1)]).run(|st| {
        st.screen.set_clear_color(color(235, 64, 52, 255));
        let mut layer = st.layers.get_mut(0).unwrap();
        layer.set_vertices([
            vec2(-180.0, 120.0),
            vec2(34.0, -174.0),
            vec2(110.0, 24.0),
        ]);
        layer.set_indices([0, 1, 2]);
        layer.set_instances([instance(Default::default(), color(252, 186, 3, 255))])
    })
}
