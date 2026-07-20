mod app;
mod state;
mod vertex;

use crate::app::App;

define_vertex! { TriangleVertex;
    position: [f32; 3],
    tex_coords: [f32; 2]
}

fn main() {
    let vertices = vec![
        TriangleVertex { position: [-1.,  1., 0.], tex_coords: [0., 0.] },
        TriangleVertex { position: [ 1.,  1., 0.], tex_coords: [1., 0.] },
        TriangleVertex { position: [ 1., -1., 0.], tex_coords: [1., 1.] },
        TriangleVertex { position: [ 1., -1., 0.], tex_coords: [1., 1.] },
        TriangleVertex { position: [-1., -1., 0.], tex_coords: [0., 1.] },
        TriangleVertex { position: [-1.,  1., 0.], tex_coords: [0., 0.] },
    ];
    let app = App::new(include_str!("shader.wgsl"), vertices);
    app.run().expect("Cannot run the app");
}

