# ketupa_demo_engine
This is the engine behind interactive examples of [ketupa](https://github.com/30bit/ketupa) crate.
It is built on top of 
[wgpu](https://wgpu.rs/), 
[winit](https://docs.rs/winit/latest/winit/) and 
[lyon](https://docs.rs/lyon/latest/lyon/).

## Hello World
```rust
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

``` 

## State
`State` is the parameter of the function passed to the `Setup::run` 
and represents mutable handle to the data associated with each frame.
```rust
pub struct State<'a> {
    pub layers: &'a mut Layers,
    pub tessellator: &'a mut Tessellator,
    pub screen: &'a mut Screen,
    pub mouse: &'a Mouse,
    pub keys: &'a Keys,
    pub delta: &'a Duration,
}

```


## Layers
- Everything drawn lives in some layer out of the ones that were registered during setup. 
- `LayerBounds` are passed during setup and specify the capacity for vertices, indices and instances. 
- `Layer`'s index corresponds to its depth, 
i.e. instances belonging to a layer with an index 0 will be drawn on top of instances
of a layer with an index 1. 
- During the frame `state.layers` can be accessed mutably as `LayerMut` by `State::get_mut` in order to
update vertices, indices and instances. 



## Tessellator
- `Tessellator` is a wrapper around 
[lyon](https://docs.rs/lyon/latest/lyon/)'s 
[`FillTessellator`](https://docs.rs/lyon_tessellation/latest/lyon_tessellation/struct.FillTessellator.html) and 
[`StrokeTessellator`](https://docs.rs/lyon_tessellation/latest/lyon_tessellation/struct.StrokeTessellator.html) api.
- `TesselatorChain` represents a sequence of polylines.

```rust
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
```
