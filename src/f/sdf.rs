
use std::{fs, io};
use crate::f;


pub struct Sdf {
  mesh: f::FMesh,
  pub shader: glium::Program,
}



impl Sdf {
  pub fn create(display: &glium::Display, filename: &str) -> f::FMesh {
    let vertices: Vec<f::Vertex> = vec! [
      f::Vertex::new( [-1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [0.0, 0.0] ), // Top-left
      f::Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [1.0, 0.0] ), // Top-right
      f::Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [1.0, 1.0] ), // Bottom-right
      f::Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [0.0, 1.0] )
    ];
    let indices: Vec<u32> = vec![ 0u32, 1, 3,  ];
    let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
    let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let fs = f::shader::load_shader( filename );
    let vs = f::shader::load_shader( "sdf-vert.glsl" );
    let shader = glium::Program::from_source(display, &vs, &fs, None).unwrap();

    f::FMesh{ vbuffer, ibuffer, material: None, vertices: None, indices: None }
  }
  
}
