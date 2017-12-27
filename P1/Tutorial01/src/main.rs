//Created feb 2016, Andrew Bednarski.
//Inspired by https://en.wikibooks.org/wiki/OpenGL_Programming/Modern_OpenGL_Introduction
//and
//https://github.com/bjz/gl-rs/blob/master/gl/examples/triangle.rs
//No warranties here, do whatever you want with this code, as long as you do not try to stop others from doing likewise.
//Also yay Cleveland Browns, next year is our year!.


//imports and use from the gl-rs example
extern crate gl;
extern crate glutin;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;


//I don't especially like globals for passing state around, but it follows the tutorial so....
static mut program : GLuint = 0 as GLuint;
static mut  attribute_coord2d : GLint = 0 as GLint;


//functions from wikibooks
fn init_resources() -> bool 
{
    unsafe
    {
        let mut compile_ok = gl::FALSE as GLint;
        let mut link_ok = gl::FALSE as GLint;
        
        let vs = gl::CreateShader(gl::VERTEX_SHADER);
        let vs_source : &'static str =
            //"#version 100\n"  // OpenGL ES 2.0
            "#version 120\n  // OpenGL 2.1
            attribute vec2 coord2d;                  
             void main(void) {                        
             gl_Position = vec4(coord2d, 0.0, 1.0); 
            }"; 
        let c_str_vs = CString::new(vs_source.as_bytes()).unwrap();
        
        gl::ShaderSource(vs, 1, &c_str_vs.as_ptr(), ptr::null());
        gl::CompileShader(vs);
        gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut compile_ok);
        if compile_ok != (gl::TRUE as GLint)
        {
            println!("Error in vertex shader");
            return false;
        }
    
    
    
    
        let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
	    let fs_source : &'static str =
		//"#version 100\n"  // OpenGL ES 2.0
		"#version 120\n  // OpenGL 2.1
		void main(void) {        
		  gl_FragColor[0] = 0.0; 
		  gl_FragColor[1] = 0.0; 
		  gl_FragColor[2] = 1.0; 
		}";

	     let c_str_fs = CString::new(fs_source.as_bytes()).unwrap();
        
          gl::ShaderSource(fs, 1, &c_str_fs.as_ptr(), ptr::null());
          gl::CompileShader(fs);
          gl::GetShaderiv(fs, gl::COMPILE_STATUS, &mut compile_ok);
          if compile_ok != (gl::TRUE as GLint)
        {    
                println!("Error in fragment shader");
                return false;
           }
    
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
	    gl::LinkProgram(program);
	    gl::GetProgramiv(program, gl::LINK_STATUS, &mut link_ok);
	    if link_ok != (gl::TRUE as GLint)
        {
		      println!("Error in glLinkProgram");
		      return false;
	    }
        
        let attribute_name : &'static str  = "coord2d";
        let c_str_at = CString::new(attribute_name.as_bytes()).unwrap();
	    attribute_coord2d = gl::GetAttribLocation(program, c_str_at.as_ptr());
	    if (attribute_coord2d == (-1 as GLint)) 
        {
		      println!("Could not bind attribute {}", attribute_name);
		      return false;
	    }
    
   }
   return true;
}

fn render(window :&glutin::Window) 
{
    unsafe
    {
        /* Clear the background as white */
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(program);
        gl::EnableVertexAttribArray(attribute_coord2d as GLuint);
        let triangle_vertices: [GLfloat; 6] = 
        [
            0.0,  0.8,
            -0.8, -0.8,
            0.8, -0.8,
        ];
        /* Describe our vertices array to OpenGL (it can't guess its format automatically) */
        gl::VertexAttribPointer(
            attribute_coord2d as GLuint, // attribute
            2 as GLint,                 // number of elements per vertex, here (x,y)
            gl::FLOAT,          // the type of each element
            gl::FALSE as GLboolean,          // take our values as-is
            0,                 // no extra data between each position
            mem::transmute(&triangle_vertices[0])  // pointer to the C array
                            );
        
        /* Push each element in buffer_vertices to the vertex shader */
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        
        gl::DisableVertexAttribArray(attribute_coord2d as GLuint);

        /* Display the result */
        window.swap_buffers().unwrap();
    }
}
fn free_resources() 
{
   unsafe 
   {
        gl::DeleteProgram(program);
   }
}

fn mainLoop(window: &glutin::Window) 
{
	 for event in (*window).wait_events() 
     {

		render(window);
        //the if let was not so clear to me, for fun lets use match!
        match event 
        {
            glutin::Event::Closed => return,
            _ => continue,
        };
     }
	
}

fn main() 
{
    //as in the gl-rs example
    let window = glutin::Window::new().unwrap();
    unsafe { window.make_current() }.unwrap();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


	/* When all init functions run without errors,
	   the program can initialise the resources */
	if (!init_resources())
    {
        std::process::exit(1);
    }

	/* We can display something if everything goes OK */
	mainLoop(&window);

	/* If the program exits in the usual way,
	   free resources and exit with a success */
	free_resources();
	 std::process::exit(0);
}
