pub trait Vertex: bytemuck::Pod {
    const LAYOUT: wgpu::VertexBufferLayout<'static>;
}

#[macro_export]
macro_rules! type_map {
    (i8) => { wgpu::VertexFormat::Sint8 };
    ([i8; 2]) => { wgpu::VertexFormat::Sint8x2 };
    ([i8; 4]) => { wgpu::VertexFormat::Sint8x4 };
    (i16) => { wgpu::VertexFormat::Sint16 };
    ([i16; 2]) => { wgpu::VertexFormat::Sint16x2 };
    ([i16; 4]) => { wgpu::VertexFormat::Sint16x4 };
    (i32) => { wgpu::VertexFormat::Sint32 };
    ([i32; 2]) => { wgpu::VertexFormat::Sint32x2 };
    ([i32; 3]) => { wgpu::VertexFormat::Sint32x3 };
    ([i32; 4]) => { wgpu::VertexFormat::Sint32x4 };
    (u8) => { wgpu::VertexFormat::Uint8 };
    ([u8; 2]) => { wgpu::VertexFormat::Uint8x2 };
    ([u8; 4]) => { wgpu::VertexFormat::Uint8x4 };
    (u16) => { wgpu::VertexFormat::Uint16 };
    ([u16; 2]) => { wgpu::VertexFormat::Uint16x2 };
    ([u16; 4]) => { wgpu::VertexFormat::Uint16x4 };
    (u32) => { wgpu::VertexFormat::Uint32 };
    ([u32; 2]) => { wgpu::VertexFormat::Uint32x2 };
    ([u32; 3]) => { wgpu::VertexFormat::Uint32x3 };
    ([u32; 4]) => { wgpu::VertexFormat::Uint32x4 };
    (f32) => { wgpu::VertexFormat::Float32 };
    ([f32; 2]) => { wgpu::VertexFormat::Float32x2 };
    ([f32; 3]) => { wgpu::VertexFormat::Float32x3 };
    ([f32; 4]) => { wgpu::VertexFormat::Float32x4 };
    (f64) => { wgpu::VertexFormat::Float64 };
    ([f64; 2]) => { wgpu::VertexFormat::Float64x2 };
    ([f64; 3]) => { wgpu::VertexFormat::Float64x3 };
    ([f64; 4]) => { wgpu::VertexFormat::Float64x4 };
}

#[macro_export]
macro_rules! vertex_attrs {
    ([$($x:expr),*]; $k:expr; $i:expr;) => { [$($x),*] };
    ([$($x:expr),*]; $k:expr; $i:expr; $type:tt, $($t:tt,)*) => {
        vertex_attrs!(
            [
                $($x,)*
                wgpu::VertexAttribute {
                    format: type_map!($type),
                    offset: $k,
                    shader_location: $i,
                }
            ];
            $k + type_map!($type).size();
            $i + 1;
            $($t,)*
        )
    };
}

#[macro_export]
macro_rules! define_vertex {
    ($name:ident; $($field:ident: $type:tt),*) => {
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct $name { $($field: $type),* }

        impl crate::vertex::Vertex for $name {
            const LAYOUT: wgpu::VertexBufferLayout<'static> =
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Self>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &vertex_attrs!([]; 0; 0; $($type,)*)
                };
        }
    };
}

