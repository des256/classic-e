extern crate gl;
extern crate glutin;
extern crate image;

use image::DynamicImage;

use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use gl::types::GLchar;
use gl::types::GLint;

mod fonts;
use fonts::Font;
use fonts::MONO;
use fonts::PIXEL;
use fonts::SANS;
use fonts::SANSI;
use fonts::SERIF;
use fonts::SERIFI;

mod grid;
pub use grid::Row;
pub use grid::Column;
mod menubar;
pub use menubar::MenuBar;

const STATIC: &str = "/home/desmond/e/static";

const UI_VERTEX_SHADER: &str = r#"
    #version 420 core

    layout(location = 0) in vec4 i_quad;
    layout(location = 1) in vec4 i_tex;
    layout(location = 2) in vec4 i_col;
    layout(location = 3) in uvec2 i_ml;  // m = mode, l = texture layer

    out Quad {
        vec4 tex;
        vec4 col;
        uvec2 ml;
    } vs_out;

    void main() {
        gl_Position = i_quad;
        vs_out.tex = i_tex;
        vs_out.col = i_col;
        vs_out.ml = i_ml;
    }
"#;

const UI_GEOMETRY_SHADER: &str = r#"
    #version 420 core

    uniform mat3x2 u_mat;  // combined transformation matrix
    uniform vec4 u_clip;   // transformed clipping rectangle

    layout(points) in;
    layout(triangle_strip, max_vertices = 4) out;

    in Quad {
        vec4 tex;
        vec4 col;
        uvec2 ml;
    } gs_in[];

    out Vertex {
        vec2 tex;
        vec4 col;
        vec4 clip;
        flat uvec2 ml;
    } gs_out;

    void main() {
        // transform source coordinates
        vec4 p = gl_in[0].gl_Position;
        p.xy = u_mat * vec3(p.xy,0.0);
        p.zw = u_mat * vec3(p.zw,0.0);

        // upper-left corner
        gl_Position = vec4(-1.0 + 2.0 * p.x,1.0 - 2.0 * p.y,0.0,1.0);
        gs_out.tex = vec2(gs_in[0].tex.x,gs_in[0].tex.y);
        gs_out.col = gs_in[0].col;
        gs_out.clip = vec4(p.x - u_clip.x,p.y - u_clip.y,u_clip.z - p.x,u_clip.w - p.y);
        gs_out.ml = gs_in[0].ml;
        EmitVertex();

        // upper-right corner
        gl_Position = vec4(-1.0 + 2.0 * (p.x + p.z),1.0 - 2.0 * p.y,0.0,1.0);
        gs_out.tex = vec2(gs_in[0].tex.x + gs_in[0].tex.z,gs_in[0].tex.y);
        gs_out.col = gs_in[0].col;
        gs_out.clip = vec4(p.x + p.z - u_clip.x,p.y - u_clip.y,u_clip.z - p.x - p.z,u_clip.w - p.y);
        gs_out.ml = gs_in[0].ml;
        EmitVertex();

        // lower-left corner            
        gl_Position = vec4(-1.0 + 2.0 * p.x,1.0 - 2.0 * (p.y + p.w),0.0,1.0);
        gs_out.tex = vec2(gs_in[0].tex.x,gs_in[0].tex.y + gs_in[0].tex.w);
        gs_out.col = gs_in[0].col;
        gs_out.clip = vec4(p.x - u_clip.x,p.y + p.w - u_clip.y,u_clip.z - p.x,u_clip.w - p.y - p.w);
        gs_out.ml = gs_in[0].ml;
        EmitVertex();

        // lower-right corner
        gl_Position = vec4(-1.0 + 2.0 * (p.x + p.z),1.0 - 2.0 * (p.y + p.w),0.0,1.0);
        gs_out.tex = vec2(gs_in[0].tex.x + gs_in[0].tex.z,gs_in[0].tex.y + gs_in[0].tex.w);
        gs_out.col = gs_in[0].col;
        gs_out.clip = vec4(p.x + p.z - u_clip.x,p.y + p.w - u_clip.y,u_clip.z - p.x - p.z,u_clip.w - p.y - p.w);
        gs_out.ml = gs_in[0].ml;
        EmitVertex();

        EndPrimitive();
    }
"#;

const UI_FRAGMENT_SHADER: &str = r#"
    #version 420 core

    in Vertex {
        vec2 tex;
        vec4 col;
        vec4 clip;
        flat uvec2 ml;
    } fs_in;

    out vec4 FragColor;

    uniform sampler2DArray sampler;
    
    float median(float r, float g, float b) {
        return max(min(r, g), min(max(r, g), b));
    }
    
    void main() {
        if(fs_in.clip.x < 0.0)
            discard;
        if(fs_in.clip.y < 0.0)
            discard;
        if(fs_in.clip.z < 0.0)
            discard;
        if(fs_in.clip.w < 0.0)
            discard;
        vec4 c = vec4(fs_in.col.b,fs_in.col.g,fs_in.col.r,fs_in.col.a);
        vec4 t = texture(sampler,vec3(fs_in.tex,fs_in.ml.y));
        switch(fs_in.ml.x)
        {
            case 0:
                discard;

            case 1:
                FragColor = c;
                break;

            case 2:
                FragColor = t;
                break;

            case 3:
                FragColor = c * t;
                break;

            case 4:
                FragColor = c + t;
                break;

            case 5:
                FragColor = c - t;
                break;

            case 6:
                FragColor = t - c;
                break;

            case 7:
                vec2 unit = (4.0 / textureSize(sampler,0)).xy;
                float dist = median(t.x,t.y,t.z) - 0.5;
                dist *= dot(unit,0.5 / fwidth(fs_in.tex));
                float cov = clamp(dist + 0.5,0.0,1.0);
                FragColor = vec4(c.x,c.y,c.z,c.w * cov);
                break;
        }
    }
"#;

pub struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(packed)]
pub struct Quad {
    pub x: f32,   // left coordinate of quad
    pub y: f32,   // top coordinate of quad
    pub xs: f32,  // width of quad
    pub ys: f32,  // height of quad
    pub u: f32,   // left texture coordinate
    pub v: f32,   // top texture coordinate
    pub us: f32,  // texture width
    pub vs: f32,  // texture height
    pub col: u32, // color
    pub m: u16,   // rendering mode
    pub l: u16,   // texture array index
}

pub struct View {
    quads: Vec<Quad>,
    vao: u32,
    mxx: f32,
    mxy: f32,
    mx1: f32,
    myx: f32,
    myy: f32,
    my1: f32,
    xs: f32,
    ys: f32,
}

impl View {
    pub fn new(x: f32, y: f32, xs: f32,ys: f32) -> View {
        let mut vao: u32 = 0;
        unsafe {
            // create vertex array for quads
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 0, 0 as *const c_void, gl::DYNAMIC_DRAW);

            let quad_stride = mem::size_of::<Quad>() as i32;
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, quad_stride, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, quad_stride, 16 as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(
                2,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                quad_stride,
                32 as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            gl::VertexAttribIPointer(3, 4, gl::UNSIGNED_SHORT, quad_stride, 36 as *const c_void);
            gl::EnableVertexAttribArray(3);
        }
        View {
            quads: Vec::new(),
            vao: vao,
            mxx: 1.0,
            mxy: 0.0,
            mx1: x,
            myx: 0.0,
            myy: 1.0,
            my1: y,
            xs: xs,
            ys: ys,
        }
    }

    fn push(&mut self, x: f32, y: f32, xs: f32, ys: f32, u: f32, v: f32, us: f32, vs: f32, col: u32, m: u16, l: u16) {
        self.quads.push(Quad {x: x, y: y, xs: xs, ys: ys, u: u, v: v, us: us, vs: vs, col: col, m: m, l: l });
    }

    fn draw(&self) {
        if self.quads.len() > 0 {
            println!("drawing {} quads for view",self.quads.len());
            let mut mat: [f32; 6] = [self.mxx,self.mxy,self.mx1,self.myx,self.myy,self.my1];
            let mut clip: [f32; 4] = [0.0,0.0,self.xs,self.ys];
            unsafe {
                gl::UniformMatrix3x2fv(0,1,gl::FALSE,mat.as_ptr());
                gl::Uniform4fv(1,1,clip.as_ptr());
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::POINTS, 0, self.quads.len() as i32);
            }
        }
    }

    fn movesize(&mut self,x: f32, y: f32, xs: f32, ys: f32) {
        self.mx1 = x;
        self.my1 = y;
        self.xs = xs;
        self.ys = ys;
    }
}

pub trait Handler {
    fn draw(&self);
    fn build(&mut self);
    fn keypress(&self,k: u32);
    fn keyrelease(&self,k: u32);
    fn mousepress(&self,x: f32,y: f32,b: u32);
    fn mouserelease(&self,x: f32,y: f32,b: u32);
    fn mousemove(&self,x: f32,y: f32);
    fn mousewheel(&self,dx: f32,dy: f32);
}

pub struct UI<'a> {
    events_loop: glutin::EventsLoop,
    context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    root: Option<&'a Handler>,
    font_tex: u32,
    sp: u32,
}

impl<'a> UI<'a> {
    pub fn new(width: u32, height: u32, title: &str) -> UI {
        let events_loop = glutin::EventsLoop::new();
        let builder = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize { width: width as f64,height: height as f64 })
            .with_title(title);
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::GlThenGles { opengl_version: (4, 5), opengles_version: (2, 0), })
            .with_vsync(true)
            .build_windowed(builder,&events_loop)
            .expect("e: unable to create OpenGL context");
        let context = unsafe { context.make_current().unwrap() };
        gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
        // create fonts
        let mut fonts: Vec<Font> = Vec::new();
        fonts.push(MONO);
        fonts.push(PIXEL);
        fonts.push(SANS);
        fonts.push(SANSI);
        fonts.push(SERIF);
        fonts.push(SERIFI);
        let mut font_tex: u32 = 0;
        let mut sp: u32 = 0;
        unsafe {
            // load fonts
            gl::GenTextures(1, &mut font_tex);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, font_tex);
            gl::TexImage3D(
                gl::TEXTURE_2D_ARRAY,
                0,
                gl::RGBA8 as i32,
                width as i32,
                height as i32,
                fonts.len() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                0 as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D_ARRAY, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D_ARRAY,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as i32,
            );
            for font in fonts {
                let img = image::open(&format!("{}/fonts/{}", STATIC, font.name)).expect("e: unable to load font image");
                let img = match img {
                    DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };
                gl::TexSubImage3D(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    0,
                    0,
                    font.page as i32,
                    font.scale_w as i32,
                    font.scale_h as i32,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.into_raw().as_ptr() as *const c_void,
                );
            }
            // compile vertex shader
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            let vcstr = CString::new(UI_VERTEX_SHADER.as_bytes()).unwrap();
            gl::ShaderSource(vs, 1, &vcstr.as_ptr(), ptr::null());
            gl::CompileShader(vs);
            let mut success = gl::FALSE as GLint;
            let mut len: GLint = 0;
            gl::GetShaderiv(vs,gl::INFO_LOG_LENGTH,&mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let mut error: CString = CString::from_vec_unchecked(buffer);
            gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderInfoLog(
                vs,
                len,
                ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            if success != gl::TRUE as GLint {
                println!("e: vertex shader:\n {}", error.to_string_lossy());
                panic!("e: aborting.");
            }
            // compile geometry shader
            let gs = gl::CreateShader(gl::GEOMETRY_SHADER);
            let gcstr = CString::new(UI_GEOMETRY_SHADER.as_bytes()).unwrap();
            gl::ShaderSource(gs, 1, &gcstr.as_ptr(), ptr::null());
            gl::CompileShader(gs);
            gl::GetShaderiv(gs,gl::INFO_LOG_LENGTH,&mut len);
            buffer = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            error = CString::from_vec_unchecked(buffer);
            gl::GetShaderiv(gs, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderInfoLog(
                gs,
                len,
                ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            if success != gl::TRUE as GLint {
                println!("e: geometry shader:\n{}", error.to_string_lossy());
                panic!("e: aborting.");
            }
            // compile fragment shader
            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fcstr = CString::new(UI_FRAGMENT_SHADER.as_bytes()).unwrap();
            gl::ShaderSource(fs, 1, &fcstr.as_ptr(), ptr::null());
            gl::CompileShader(fs);
            gl::GetShaderiv(fs,gl::INFO_LOG_LENGTH,&mut len);
            buffer = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            error = CString::from_vec_unchecked(buffer);
            gl::GetShaderiv(fs, gl::COMPILE_STATUS, &mut success);
            gl::GetShaderInfoLog(
                fs,
                len,
                ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            if success != gl::TRUE as GLint {
                println!("e: fragment shader:\n{}", error.to_string_lossy());
                panic!("e: aborting.");
            }
            // link shaders
            sp = gl::CreateProgram();
            gl::AttachShader(sp, vs);
            gl::AttachShader(sp, gs);
            gl::AttachShader(sp, fs);
            gl::LinkProgram(sp);
            gl::GetProgramiv(sp, gl::INFO_LOG_LENGTH,&mut len);
            buffer = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            error = CString::from_vec_unchecked(buffer);
            gl::GetProgramiv(sp, gl::LINK_STATUS, &mut success);
            gl::GetProgramInfoLog(
                sp,
                len,
                ptr::null_mut(),
                error.as_ptr() as *mut GLchar,
            );
            if success != gl::TRUE as GLint {
                println!("e: shader program:\n{}", error.to_string_lossy());
                panic!("e: aborting.");
            }
            // and delete references to the separate shaders
            gl::DeleteShader(vs);
            gl::DeleteShader(gs);
            gl::DeleteShader(fs);
        }
        UI {
            events_loop: events_loop,
            context: context,
            root: None,
            font_tex: font_tex,
            sp: sp,
        }
    }

    pub fn root(&mut self,handler: &'a mut Handler) {
        self.root = Some(handler);
    }

    pub fn handle(&mut self) -> bool {
        let mut running = true;
        let root = &mut self.root;
        let sp = self.sp;
        self.events_loop.poll_events(|event|
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::Resized(_s) => {
                        match root {
                            Some(w) => { w.build(); },  // TODO: set size as well
                            None => (),
                        }
                    },
                    glutin::WindowEvent::Moved(_p) => (),
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Destroyed => (),
                    glutin::WindowEvent::DroppedFile(_p) => (),
                    glutin::WindowEvent::HoveredFile(_p) => (),
                    glutin::WindowEvent::HoveredFileCancelled => (),
                    glutin::WindowEvent::ReceivedCharacter(_c) => (),
                    glutin::WindowEvent::Focused(_f) => (),
                    glutin::WindowEvent::KeyboardInput { device_id: _, input: _ } => (),
                    glutin::WindowEvent::CursorMoved { device_id: _, position: _, modifiers: _ } => (),
                    glutin::WindowEvent::CursorEntered { device_id: _ } => (),
                    glutin::WindowEvent::CursorLeft { device_id: _ } => (),
                    glutin::WindowEvent::MouseWheel { device_id: _, delta: _, phase: _, modifiers: _ } => (),
                    glutin::WindowEvent::MouseInput { device_id: _, state: _, button: _, modifiers: _ } => (),
                    glutin::WindowEvent::TouchpadPressure { device_id: _, pressure: _, stage: _ } => (),
                    glutin::WindowEvent::AxisMotion { device_id: _, axis: _, value: _ } => (),
                    glutin::WindowEvent::Refresh => {
                        match root {
                            Some(w) => unsafe {
                                gl::Enable(gl::BLEND);
                                gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
                                gl::BlendFuncSeparate(
                                    gl::SRC_ALPHA,
                                    gl::ONE_MINUS_SRC_ALPHA,
                                    gl::ONE,
                                    gl::ONE_MINUS_SRC_ALPHA,
                                );
                                gl::UseProgram(sp);
                                w.draw();
                                gl::Disable(gl::BLEND);
                            },
                            None => (),
                        }
                    },
                    glutin::WindowEvent::Touch(_t) => (),
                    glutin::WindowEvent::HiDpiFactorChanged(_f) => (),
                }
            }
        );
        running
    }
}
