
use crate::f;
extern crate glium;


pub struct Sdf {
  pub mesh: f::FMesh,
  pub shader: glium::Program,
}


pub fn create(display: &glium::Display, filename: &str) -> Sdf {
  let vertices = [
    f::Vertex::new( [-1.0f32,  1.0, 0.0], [1.0,0.0,0.0], [0.0, 0.0] ), // Top-left
    f::Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,1.0,0.0], [1.0, 0.0] ), // Top-right
    f::Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [1.0, 1.0] ), // Bottom-right
    f::Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [0.0, 1.0] )
  ];
  //let indices: Vec<u32> = vec![ 0u32,1,3, 0,1,2, 1,2,3, 3,2,1, 2,1,3, 2,3,1, 0,3,2, 0,2,1 ];
  let indices = [ 0u32,1,3, 0,1,2, 1,2,3, 3,2,1, 2,1,3, 2,3,1, 0,3,2, 0,2,1 ];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();


  let vv = glium::VertexBuffer::new(display,
    &[
        f::Vertex { position: [-0.5f32, -0.5, 0.0], normal: [0.0f32, 1.0, 0.0], tex_coords: [0.0f32, 0.0] },
        f::Vertex { position: [ 0.0f32,  0.5, 0.0], normal: [0.0f32, 0.0, 1.0], tex_coords: [0.0f32, 0.0] },
        f::Vertex { position: [ 0.5f32, -0.5, 0.0], normal: [1.0f32, 0.0, 0.0], tex_coords: [0.0f32, 0.0] },
    ]
  ).unwrap();
  let ii = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                                   &[0u32, 1, 2]).unwrap();
    


  let fs = f::Shader::load_shader( filename );
  //let vs = f::Shader::load_shader( "sdf-vert.glsl" );
  let vs = f::Shader::load_shader( "test-vert.glsl" );

  println!("____vS: {:?}", vs);
  println!("____FS: {:?}", fs);

  let shader = glium::Program::from_source(display, &vs, &fs, None).unwrap();
  let mesh = f::FMesh{ vbuffer: vv, ibuffer: ii, material: None, vertices: None, indices: None };
  Sdf {
    mesh, shader
  }
}
