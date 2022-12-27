pub mod f::shader;
use std::{fs};


// create vertex & fragment shaders, assuming files are  "data/shaders/" + filename + "-vert/frag.glsl"
pub fn create_shader_vf( display: &glium::Display, filename: &str ) -> glium::Program
{
  let vs = load_shader( &[filename, "-vert.glsl"].concat() );
  let fs = load_shader( &[filename, "-frag.glsl"].concat() );
  glium::Program::from_source(display, &vs, &fs, None).unwrap()
}


fn include_files(v: &str) -> std::string::String {
  let mut vs = String::from(v);
  let re = regex::Regex::new(r"#include\s*[<]?([_a-zA-Z0-9._-]*).*").unwrap();
  for cap in re.captures_iter( &String::from(v) ) {
    println!("___ finding {:?}", &cap);
    let inc = fs::read_to_string( ["data/shaders/", &cap[1]].concat() ).expect("Shader include file is missing");
    vs = re.replace_all(&vs, inc).into_owned();
  }
  vs
}

pub fn load_shader(filename: &str) -> std::string::String {
  println!("___ load shader {:?}", filename);
  let pv = ["data/shaders/", filename].concat();
  let vs = fs::read_to_string(pv).expect("Something went wrong reading the shader");
  include_files(&vs)
}
