use std::ffi::c_void;

// This is called when the Javascript initializes

#[no_mangle]
pub extern "C" fn start() {
    unsafe {
        imported::setup_canvas();

        // Create the vertex buffer
        let vertex_buffer = imported::create_buffer();
        imported::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);
        buffer_data_f32(
            GLEnum::ArrayBuffer,
            &[-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0],
            GLEnum::StaticDraw,
        );

        // Crate the index buffer
        let index_buffer = imported::create_buffer();
        imported::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);

        // There is a WebGL extension to use u32 indices, but here we'll just use u16.
        buffer_data_u16(GLEnum::ElementArrayBuffer, &[0, 1, 2], GLEnum::StaticDraw);

        // Create the vertex shader
        let vertex_shader = imported::create_shader(GLEnum::VertexShader);
        shader_source(
            vertex_shader,
            r#"
            attribute vec3 vertex_position;

            void main(void) {
                gl_Position = vec4(vertex_position, 1.0);
            }
        "#,
        );
        imported::compile_shader(vertex_shader);

        // Create the fragment shader
        let fragment_shader = imported::create_shader(GLEnum::FragmentShader);
        shader_source(
            fragment_shader,
            r#"void main() {
                gl_FragColor = vec4(1.0, 0.5, 0.313, 1.0);
              }
          "#,
        );
        imported::compile_shader(fragment_shader);

        // Create the shader program
        let shader_program = imported::create_program();
        imported::attach_shader(shader_program, vertex_shader);
        imported::attach_shader(shader_program, fragment_shader);
        imported::link_program(shader_program);
        imported::use_program(shader_program);

        // Get the location of the vertex_position attribute.
        // This is needed to later bind the data to it.
        let attrib_location = get_attrib_location(shader_program, "vertex_position").unwrap();

        // Enable the attribute location.
        // This only needs to be done once per program, it's really just boilerplate.
        imported::enable_vertex_attrib_array(attrib_location);

        // Bind the vertex buffer. It's already bound, but in a more regular draw loop
        // it would not be.
        imported::bind_buffer(GLEnum::ArrayBuffer, vertex_buffer);

        // Now bind the vertex attributed to the correct buffer.
        imported::vertex_attrib_pointer(attrib_location as u32, 3, GLEnum::Float, false, 0, 0);

        // Bind the index buffer as well.
        imported::bind_buffer(GLEnum::ElementArrayBuffer, index_buffer);

        imported::clear_color(0.3765, 0.3137, 0.8627, 1.0);
        imported::clear(GLEnum::ColorBufferBit);
        imported::draw_elements(GLEnum::Triangles, 3, GLEnum::UnsignedShort, 0);
    }
}

// A few of the external functions we'll wrap so that we can use them in a more Rusty way.

pub fn shader_source(shader: JSObject, source: &str) {
    unsafe { imported::shader_source(shader, source as *const str as *const c_void, source.len()) }
}

pub fn get_attrib_location(program: JSObject, name: &str) -> Option<GLUint> {
    unsafe {
        // Returns -1 if attribute does not exist, to be more Rusty we'll return None instead.
        // A GLUint is returned instead because all APIs that consume this value expect a GLUint.
        // getAttribLocation only returns a signed value to use the -1 to indicate if the location
        // does not exist.
        let result =
            imported::get_attrib_location(program, name as *const str as *const c_void, name.len());
        if result == -1 {
            None
        } else {
            Some(result as u32)
        }
    }
}

pub fn bind_buffer(target: GLEnum, gl_object: Option<JSObject>) {
    unsafe { imported::bind_buffer(target, gl_object.unwrap_or(JSObject::null())) }
}

pub fn buffer_data_f32(target: GLEnum, data: &[f32], usage: GLEnum) {
    unsafe {
        imported::buffer_data_f32(
            target,
            data as *const [f32] as *const c_void,
            data.len(),
            usage,
        )
    }
}

pub fn buffer_data_u16(target: GLEnum, data: &[u16], usage: GLEnum) {
    unsafe {
        imported::buffer_data_u16(
            target,
            data as *const [u16] as *const c_void,
            data.len(),
            usage,
        )
    }
}

// The raw external bindings to Javascript
mod imported {
    use super::*;

    extern "C" {
        pub fn setup_canvas();
        //   pub fn create_shader(source: *const c_void, source_length: usize);
        pub fn create_buffer() -> JSObject;
        pub fn bind_buffer(target: GLEnum, gl_object: JSObject);
        pub fn buffer_data_f32(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );

        pub fn buffer_data_u16(
            target: GLEnum,
            data: *const c_void,
            data_length: usize,
            usage: GLEnum,
        );

        pub fn create_shader(shader_type: GLEnum) -> JSObject;
        pub fn shader_source(shader: JSObject, source: *const c_void, source_length: usize);
        pub fn compile_shader(shader: JSObject);
        pub fn create_program() -> JSObject;
        pub fn attach_shader(program: JSObject, shader: JSObject);
        pub fn link_program(program: JSObject);
        pub fn use_program(program: JSObject);
        pub fn get_attrib_location(
            program: JSObject,
            name: *const c_void,
            name_length: usize,
        ) -> GLint;
        pub fn vertex_attrib_pointer(
            index: GLUint,
            size: GLint,
            _type: GLEnum,
            normalized: bool,
            stride: GLsizei,
            pointer: GLintptr,
        );
        pub fn enable_vertex_attrib_array(index: GLUint);
        pub fn clear_color(r: f32, g: f32, b: f32, a: f32);
        pub fn clear(mask: GLEnum);
        pub fn draw_elements(mode: GLEnum, count: GLsizei, _type: GLEnum, offset: GLintptr);
    }
}

// What follows are types defined to help communicate with Javascript code.

#[derive(Clone, Copy)]
#[repr(C)]
pub struct JSObject(u32);

impl JSObject {
    pub const fn null() -> Self {
        JSObject(0)
    }
}

// Sourced from here: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Constants
#[repr(u32)]
pub enum GLEnum {
    Triangles = 0x0004,
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    VertexShader = 0x8B31,
    FragmentShader = 0x8B30,
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406,
    StaticDraw = 0x88E4,
    DynamicDraw = 0x88E8,
    ColorBufferBit = 0x00004000,
}

// Define some types to make it easier to port directly from the spec.
pub type GLUint = u32;
pub type GLint = i32;
pub type GLsizei = i32;

// GLintptr should be an i64, but Wasm can't pass those so for now just use an i32.
pub type GLintptr = i32;
