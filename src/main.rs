mod app;
mod state;
mod vertex;

use crate::app::App;

define_vertex! { TriangleVertex;
    position: [f32; 3]
}

fn main() {
    let vertices = vec![
        TriangleVertex { position: [-1.,  1., 0.] },
        TriangleVertex { position: [ 1.,  1., 0.] },
        TriangleVertex { position: [ 1., -1., 0.] },
        TriangleVertex { position: [ 1., -1., 0.] },
        TriangleVertex { position: [-1., -1., 0.] },
        TriangleVertex { position: [-1.,  1., 0.] },
    ];
    let app = App::new(include_str!("shader.wgsl"), vertices);
    app.run().expect("Cannot run the app");
}

