use std::{fs, io};


pub fn load_shader_vf( filename: &str, display: &glium::Display ) -> glium::Program
{
  let pv = ["data/shaders/", filename, "-vert.glsl"].concat();
  let vs = fs::read_to_string(pv)
        .expect("Something went wrong reading the vert shader");
  let pf = ["data/shaders/", filename, "-frag.glsl"].concat();
  let fs = fs::read_to_string(pf)
        .expect("Something went wrong reading the vert shader");
  glium::Program::from_source(display, &vs, &fs, None).unwrap()
}


fn include_files(v: &str) -> std::string::String {
  let mut vs = String::from(v);
  let re = regex::Regex::new(r"#include\s*[<]?([a-zA-Z0-9._-]*).*").unwrap();
  for cap in re.captures_iter( &String::from(v) ) {
    let inc = fs::read_to_string( ["data/shaders/", &cap[1]].concat() ).expect("Shader include file is missing");
    //vs = &re.replace_all(&vs, inc).into_owned();
    vs = re.replace_all(&vs, inc).into_owned();
  }
  vs
}

pub fn load_shader(filename: &str) -> std::string::String {
  let pv = ["data/shaders/", filename].concat();
  let vs = fs::read_to_string(pv).expect("Something went wrong reading the vert shader");
  let re = regex::Regex::new(r"#include\s*[<]?([a-zA-Z0-9._-]*).*").unwrap();
  if re.is_match(&vs) {
    return include_files(&vs);
  }
  vs
}
