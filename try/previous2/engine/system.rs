// e::canvas::Canvas for Linux/GLX/OpenGL45
// by Desmond Germans, 2019

// code dump of old engine-specific code

/*
use std::{ffi::{CStr,CString},os::raw::{c_int,c_void},mem};
#[doc(no_inline)]
extern crate x11;
use x11::{glx,xlib::*};
#[doc(no_inline)]
extern crate gl;
use gl::types::{GLuint,GLfloat};

use crate::{Shader,Framebuffer};

const GLX_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const GLX_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;

type GlXCreateContextAttribsARBProc = unsafe extern "C" fn(dpy: *mut Display,fbc: glx::GLXFBConfig,share_context: glx::GLXContext,direct: Bool,attribs: *const c_int) -> glx::GLXContext;

fn load_function(name: &str) -> *mut c_void {
    let newname = CString::new(name).unwrap();
    let pointer: *mut c_void = unsafe { std::mem::transmute(glx::glXGetProcAddress(newname.as_ptr() as *const u8)) };
    if pointer.is_null() { panic!("Canvas: unable to access {}", name); }
    pointer
}

pub fn uniform_location(sp: GLuint,name: &str) -> i32 {
    let s = CString::new(name).expect("CString error??");
    let res = unsafe { gl::GetUniformLocation(sp,s.as_ptr()) };
    if res < 0 {
        panic!("Canvas: \"{}\" uniform not found (result: {})",name,res);
    }
    res
}

static QUAD: [GLfloat; 8] = [
    0.0,0.0,
    1.0,0.0,
    1.0,1.0,
    0.0,1.0,
];


pub struct Canvas {

    // specifications of the screen
    pub width: u32,
    pub height: u32,
    pub aspect: f32,

    // general library of things available for reference from the screen
    pub layer_shader: Shader,
    pub final_shader: Shader,
    pub copy_shader: Shader,
    pub text_shader: Shader,
    pub quad_vbo: GLuint,

    // screen internals
    connection: xcb::Connection,
    _screen: i32,
    window: u32,
    colormap: u32,
    _wm_protocols: u32,
    _wm_delete_window: u32,
    context: glx::GLXContext,
    framebuffer: Framebuffer,
}

impl Video {

    /// Create the main video interface.
    /// # Arguments
    /// * `width` - Initial width of the video interface.
    /// * `height` - Initial height of the video interface.
    /// * `aspect` - Aspect ratio (width/height) that should be maintained.
    /// # Returns
    /// Newly created video interface.
    pub fn new(width: u32,height: u32,aspect: f32,title: &str) -> Video { // TODO: should be Result<Video>

        // open connection and switch to xcb
        let (connection, _screen_number) = xcb::Connection::connect_with_xlib_display().unwrap();
        connection.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

        // verify glX version
        let mut glxmaj: c_int = 0;
        let mut glxmin: c_int = 0;
        unsafe { if glx::glXQueryVersion(connection.get_raw_dpy(),&mut glxmaj as *mut c_int,&mut glxmin as *mut c_int) == 0 { panic!("Video: unable to get glX version"); } }
        if (glxmaj * 100 + glxmin) < 103 { panic!("Video: glX version {}.{} needs to be at least 1.3",glxmaj,glxmin); }

        // obtain framebuffer context
        let attribs = [
            glx::GLX_X_RENDERABLE,  1,
            glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
            glx::GLX_RENDER_TYPE,   glx::GLX_RGBA_BIT,
            glx::GLX_X_VISUAL_TYPE, glx::GLX_TRUE_COLOR,
            glx::GLX_RED_SIZE,      8,
            glx::GLX_GREEN_SIZE,    8,
            glx::GLX_BLUE_SIZE,     8,
            glx::GLX_ALPHA_SIZE,    8,
            glx::GLX_DEPTH_SIZE,    24,
            glx::GLX_STENCIL_SIZE,  8,
            glx::GLX_DOUBLEBUFFER,  1,
            0,
        ];
        let mut fbcount: c_int = 0;
        let fbconfigs = unsafe { glx::glXChooseFBConfig(connection.get_raw_dpy(),0,attribs.as_ptr(),&mut fbcount as *mut c_int) };
        if fbcount == 0 { panic!("Video: unable to find framebuffer config"); }
        let fbconfig = unsafe { *fbconfigs };
        unsafe { XFree(fbconfigs as *mut c_void) };
        let visual = unsafe { glx::glXGetVisualFromFBConfig(connection.get_raw_dpy(), fbconfig) };
        let screen = unsafe { (*visual).screen };
        let visual_screen = connection.get_setup().roots().nth(screen as usize).unwrap();
        let depth = unsafe { (*visual).depth };
        let visualid = unsafe { (*visual).visualid };

        // create window
        let colormap = connection.generate_id();
        let window = connection.generate_id();
        xcb::create_colormap(&connection,xcb::COLORMAP_ALLOC_NONE as u8,colormap,visual_screen.root(),visualid as u32);
        let values = [
            (xcb::CW_BACK_PIXEL,visual_screen.white_pixel()),
            (xcb::CW_BORDER_PIXEL,visual_screen.black_pixel()),
            (xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE
                | xcb::EVENT_MASK_KEY_PRESS
                | xcb::EVENT_MASK_KEY_RELEASE
                | xcb::EVENT_MASK_BUTTON_PRESS
                | xcb::EVENT_MASK_BUTTON_RELEASE
                | xcb::EVENT_MASK_POINTER_MOTION
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY
            ),
            (xcb::CW_COLORMAP, colormap),
        ];
        xcb::create_window(&connection,depth as u8,window,visual_screen.root(),0,0,width as u16,height as u16,0,xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,visualid as u32,&values);

        // set window title
        xcb::change_property(&connection,xcb::PROP_MODE_REPLACE as u8,window,xcb::ATOM_WM_NAME,xcb::ATOM_STRING,8,title.as_bytes());

        // get WM_PROTOCOLS and WM_DELETE_WINDOW atoms
        let (wm_protocols, wm_delete_window) = {
            let protocols_com = xcb::intern_atom(&connection, false, "WM_PROTOCOLS");
            let delete_window_com = xcb::intern_atom(&connection, false, "WM_DELETE_WINDOW");
            let protocols = match protocols_com.get_reply() {
                Ok(protocols) => protocols.atom(),
                Err(_) => panic!("Video: unable to access WM_PROTOCOLS"),
            };
            let delete_window = match delete_window_com.get_reply() {
                Ok(delete_window) => delete_window.atom(),
                Err(_) => panic!("Video: unable to access WM_DELETE_WINDOW"),
            };
            (protocols,delete_window)
        };

        // hook WM_DELETE_WINDOW
        let protocol_set = [wm_delete_window];
        xcb::change_property(&connection,xcb::PROP_MODE_REPLACE as u8,window,wm_protocols,xcb::ATOM_ATOM,32,&protocol_set);

        // map window
        xcb::map_window(&connection, window);
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(),False) };

        // get glX extensions
        let extensions = unsafe { CStr::from_ptr(glx::glXQueryExtensionsString(connection.get_raw_dpy(),screen)) }.to_str().unwrap();
        //println!("glx extensions: {}", extensions);

        // create OpenGL context and make it current
        let mut found = false;
        for extension in extensions.split(" ") {
            if extension == "GLX_ARB_create_context" {
                found = true;
                break;
            }
        }
        if !found { panic!("Video: unable to access GLX_ARB_create_context"); }
        let glx_create_context_attribs: GlXCreateContextAttribsARBProc = unsafe { std::mem::transmute(load_function("glXCreateContextAttribsARB")) };
        let context_attribs: [c_int; 5] = [
            GLX_CONTEXT_MAJOR_VERSION_ARB as c_int, 4,
            GLX_CONTEXT_MINOR_VERSION_ARB as c_int, 5,
            0,
        ];
        let context = unsafe { glx_create_context_attribs(connection.get_raw_dpy(),fbconfig,std::ptr::null_mut(),True,&context_attribs[0] as *const c_int) };
        connection.flush();
        unsafe { XSync(connection.get_raw_dpy(), False) };
        if context.is_null() { panic!("Video: unable to open OpenGL context"); }
        if unsafe { glx::glXIsDirect(connection.get_raw_dpy(), context) } == 0 { panic!("Video: OpenGL context is indirect"); }
        unsafe { glx::glXMakeCurrent(connection.get_raw_dpy(), window as XID, context) };

        // load OpenGL symbols
        gl::load_with(|symbol| load_function(&symbol));

        // OpenGL4 doesn't allow for non-VAO processing, but GLES and WebGL have to, so, "just create a VAO and forget about it"
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }

        // create layer shader
        let layer_vs = r#"
            #version 420 core
            uniform vec4 u_rect;
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,1.0 - v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * (u_rect.x + u_rect.z * v_pos.x),1.0 - 2.0 * (u_rect.y + u_rect.w * v_pos.y),0.0,1.0);
            }
        "#;
        let layer_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex.st);
            }
        "#;
        let layer_shader = Shader::new(layer_vs,layer_fs);

        // create final shader
        let final_vs = r#"
            #version 420 core
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,1.0 - v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,1.0 - 2.0 * v_pos.y,0.0,1.0);
            }
        "#;
        let final_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            layout(location = 0) out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex.st);
            }
        "#;
        let final_shader = Shader::new(final_vs,final_fs);

        // create copy shader
        let copy_vs = r#"
            #version 420 core
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                f_tex = vec2(v_pos.x,v_pos.y);
                gl_Position = vec4(-1.0 + 2.0 * v_pos.x,-1.0 + 2.0 * v_pos.y,0.0,1.0);
            }
        "#;
        let copy_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            in vec2 f_tex;
            layout(location = 0) out vec4 fs_output;
            void main() {
                fs_output = texture2D(u_texture,f_tex.st);
            }
        "#;
        let copy_shader = Shader::new(copy_vs,copy_fs);

        // create text shader
        let text_vs = r#"
            #version 420 core
            uniform vec4 u_chars[1024];
            layout(location = 0) in vec2 v_pos;
            out vec2 f_tex;
            void main() {
                vec4 pos = u_chars[2 * gl_InstanceID];
                vec4 tex = u_chars[2 * gl_InstanceID + 1];
                f_tex = vec2(tex.x + v_pos.x * tex.z,tex.y + v_pos.y * tex.w);
                gl_Position = vec4(-1.0 + 2.0 * (pos.x + v_pos.x * pos.z),1.0 - 2.0 * (pos.y + v_pos.y * pos.w),0.0,1.0);
            }
        "#;
        let text_fs = r#"
            #version 420 core
            uniform sampler2D u_texture;
            uniform vec4 u_color;
            in vec2 f_tex;
            layout(location = 0) out vec4 fs_out;
            float median(float r, float g, float b) { return max(min(r, g), min(max(r, g), b)); }
            void main() {
                vec2 unit = (4.0 / textureSize(u_texture,0)).xy;
                vec4 t = texture2D(u_texture,f_tex);
                float dist = median(t.r,t.g,t.b) - 0.5;
                dist *= dot(unit,0.5 / fwidth(f_tex));
                float cov = clamp(dist + 0.5,0.0,1.0);
                //fs_out = vec4(t.rgb,1.0);
                fs_out = u_color * cov;
                //fs_out = vec4(1.0,1.0,1.0,1.0);
            }
        "#;
        let text_shader = Shader::new(text_vs,text_fs);

        // create quad vertices
        let mut quad_vbo: u32 = 0;
        unsafe {
            gl::GenBuffers(1,&mut quad_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER,quad_vbo);
            gl::BufferData(gl::ARRAY_BUFFER,32,mem::transmute(&QUAD[0]),gl::STATIC_DRAW);
        }

        // find exact dimensions under aesthetic aspect ratio
        let mut xs = width as f32;
        let mut ys = height as f32;
        let ys_for_xs = width as f32 / aspect;
        let xs_for_ys = height as f32 * aspect;
        if ys_for_xs > ys {
            xs = xs_for_ys;
        }
        else {
            ys = ys_for_xs;
        }

        // create framebuffer from that
        let framebuffer = Framebuffer::new(xs as u32,ys as u32);

        Video {
            width: width,
            height: height,
            aspect: aspect,
            layer_shader: layer_shader,
            final_shader: final_shader,
            copy_shader: copy_shader,
            text_shader: text_shader,
            quad_vbo: quad_vbo,
            connection: connection,
            _screen: screen,
            window: window,
            colormap: colormap,
            _wm_protocols: wm_protocols,
            _wm_delete_window: wm_delete_window,
            context: context,
            framebuffer: framebuffer,
        }
    }

    /// Render framebuffer to screen.
    pub fn present(&self) {

        unsafe {

            // while testing: dark gray where no pixels were drawn
            gl::BindFramebuffer(gl::FRAMEBUFFER,0);
            gl::Viewport(0,0,self.width as i32,self.height as i32);
            gl::Scissor(0,0,self.width as i32,self.height as i32);
            gl::ClearColor(0.5,0.5,0.5,1.0);
            gl::ClearDepth(0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // render final framebuffer onto screen
            if self.framebuffer.width == self.width {
                let dy = self.height - self.framebuffer.height;
                let y0 = dy / 2;
                gl::Viewport(0,y0 as i32,self.width as i32,self.framebuffer.height as i32);
                gl::Scissor(0,y0 as i32,self.width as i32,self.framebuffer.height as i32);
            }
            else {
                let dx = self.width - self.framebuffer.width;
                let x0 = dx / 2;
                gl::Viewport(x0 as i32,0,self.framebuffer.width as i32,self.height as i32);
                gl::Scissor(x0 as i32,0,self.framebuffer.width as i32,self.height as i32);
            }
            gl::BindTexture(gl::TEXTURE_2D,self.framebuffer.tex);
            gl::UseProgram(self.final_shader.sp);
            gl::Uniform1i(uniform_location(self.final_shader.sp,"u_texture"),0);
            gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
            gl::DisableVertexAttribArray(0);

            // done
            gl::Flush();
            glx::glXSwapBuffers(self.connection.get_raw_dpy(),self.window as XID);
        }
    }

    /// Render all layers to framebuffer.
    /// # Arguments
    /// * `layers` - All currently visible layers in the interface.
    pub fn render_framebuffer(&self,layers: &Vec<&Layer>) {

        unsafe {

            // render layers onto final framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER,self.framebuffer.fbo);
            gl::Viewport(0,0,self.framebuffer.width as i32,self.framebuffer.height as i32);
            gl::Scissor(0,0,self.framebuffer.width as i32,self.framebuffer.height as i32);

            // while testing: light gray where no pixels were drawn
            gl::ClearColor(0.8,0.8,0.8,1.0);
            gl::ClearDepth(0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for layer in layers {
                
                // TODO: configure blending according to layer specification
                // TODO: if layer alpha is not 1.0 and blend mode is not REPLACE: glEnable(GL_BLEND); glBlendFunc(...);
                gl::BindTexture(gl::TEXTURE_2D,layer.framebuffer.tex);
                gl::UseProgram(self.layer_shader.sp);
                gl::Uniform4f(uniform_location(self.layer_shader.sp,"u_rect"),layer.x0,layer.y0,layer.xs,layer.ys);
                gl::Uniform1i(uniform_location(self.layer_shader.sp,"u_texture"),0);
                gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
                gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                gl::DisableVertexAttribArray(0);
                // TODO: glDisable(GL_BLEND);
            }
        }

        self.present();
    }

    /// Handle UI events.
    /// # Returns
    /// True to keep going, false if user wanted to quit.
    pub fn handle_events(&mut self) -> bool {
        loop {
            let event = self.connection.poll_for_event();
            match event {
                None => { break; }
                Some(event) => {
                    let r = event.response_type() & !0x80;
                    match r {

                        xcb::EXPOSE => {
                            // only reshow the framebuffer as result of the expose
                            self.present();
                        },

                        xcb::KEY_PRESS => {
                            let key_press : &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                            println!("Key '{}' pressed", key_press.detail());
                        },

                        xcb::CONFIGURE_NOTIFY => {
                            // resize screen and recreate framebuffer
                            let configure_notify : &xcb::ConfigureNotifyEvent = unsafe { xcb::cast_event(&event) };
                            self.width = configure_notify.width() as u32;
                            self.height = configure_notify.height() as u32;
                            let mut xs = self.width as f32;
                            let mut ys = self.height as f32;
                            let ys_for_xs = self.width as f32 / self.aspect;
                            let xs_for_ys = self.height as f32 * self.aspect;
                            if ys_for_xs > ys {
                                xs = xs_for_ys;
                            }
                            else {
                                ys = ys_for_xs;
                            }

                            // create new framebuffer
                            let framebuffer = Framebuffer::new(xs as u32,ys as u32);

                            // scale whatever was in the other framebuffer into this one
                            unsafe {
                                gl::BindFramebuffer(gl::FRAMEBUFFER,framebuffer.fbo);
                                gl::Viewport(0,0,framebuffer.width as i32,framebuffer.height as i32);
                                gl::Scissor(0,0,framebuffer.width as i32,framebuffer.height as i32);
                                gl::BindTexture(gl::TEXTURE_2D,self.framebuffer.tex);
                                gl::UseProgram(self.copy_shader.sp);
                                gl::Uniform1i(uniform_location(self.copy_shader.sp,"u_texture"),0);
                                gl::BindBuffer(gl::ARRAY_BUFFER,self.quad_vbo);
                                gl::EnableVertexAttribArray(0);
                                gl::VertexAttribPointer(0,2,gl::FLOAT,gl::FALSE,0,0 as *const c_void);
                                gl::DrawArrays(gl::TRIANGLE_FAN,0,4);
                                gl::DisableVertexAttribArray(0);
                            }

                            // and replace the framebuffer
                            self.framebuffer = framebuffer;
                        },

                        xcb::CLIENT_MESSAGE => {
                            let client_message : &xcb::ClientMessageEvent = unsafe { xcb::cast_event(&event) };
                            let data = &client_message.data().data;
                            let atom = (data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16) | ((data[3] as u32) << 24);

                            // close window makes this function return false
                            if atom == self._wm_delete_window {
                                return false;
                            }
                        },

                        _ => {}
                    }
                }
            }
        }
        true
    }
}

impl Drop for Video {

    /// Destroy the main video interface.
    fn drop(&mut self) {
        unsafe { glx::glXDestroyContext(self.connection.get_raw_dpy(),self.context) };
        xcb::unmap_window(&self.connection,self.window);
        xcb::destroy_window(&self.connection,self.window);
        xcb::free_colormap(&self.connection,self.colormap);
    }
}


// e::audio::Audio: main audio subsystem
// by Desmond Germans, 2019

#[doc(no_inline)]
extern crate alsa;
use alsa::{PCM,pcm::{HwParams,Format,Access},PollDescriptors};
use std::{thread,sync::mpsc,os::raw::c_int};
use crate::{Track,Effect,Filter,Synth,Patch};

enum AudioMsg {
    Terminate,
    Silence,
    Track(Track),  // track instance
    Effect(Effect),  // effect instance
    TrackVolume(f32),  // volume (0..1)
    TrackPanning(f32),  // panning (-1..1)
    EffectVolume(f32),  // volume (0..1)
    EffectPanning(f32),  // panning (-1..1)
    EffectFilter(Option<Filter>),  // filter instance
    EditSetTrack(Track),  // track instance
    EditGetTrack,
    EditPlay,
    EditPause,
    EditStop,
    EditSpeed(u32),  // speed (bpm)
    EditChannels(u32),  // number of channels
    EditClear(u64,u64),  // pos, length
    EditChannelClear(u32,u64,u64),  // channel, pos, length
    EditChannelVolume(u32,f32),  // channel, volume (0..1)
    EditChannelPanning(u32,f32),  // channel, panning (-1..1)
    EditChannelEnabled(u32,bool),  // channel, enabled (False = muted, True = normal)
    EditChannelPatch(u32,Patch),  // channel, patch
    EditChannelNote(u32,u64,usize,f32,u64),  // channel, position, note, velocity (0..1), length
    EditSetPatch(Patch),  // patch
    EditGetPatch,
    EditNote(usize,f32),  // note, velocity
    EditRelease(usize),
}

enum AudioReturnMsg {
    Track(Track),  // track instance
    Patch(Patch),  // patch instance
}

#[derive(Copy, Clone)]
pub struct Frame {
    pub l: f32,
    pub r: f32,
}

pub struct Audio {
    mixer_thread: Option<thread::JoinHandle<()>>,
    tx: mpsc::Sender<AudioMsg>,
    rx_return: mpsc::Receiver<AudioReturnMsg>,
}

fn clamp(v: f32, min: f32, max: f32) -> f32 {

    if v < min { return min; }
    if v > max { return max; }
    v
}

impl Audio {

    pub fn new() -> Audio {  // TODO: should be Result<Audio>

        // create communication channel
        let (tx, rx) = mpsc::channel();
        let (_tx_return, rx_return) = mpsc::channel();

        // start mixer thread
        let mixer_thread = Some(thread::spawn(move || {

            // open device
            let pcm_maybe = PCM::new("hw:0,0",alsa::Direction::Playback,false);
            let pcm = match pcm_maybe {
                Ok(pcm) => { pcm },
                Err(_e) => { panic!("Audio: unable to open PCM device"); },
            };

            // set parameters
            let mut fps: f32 = 44100.0;
            {
                let hwp = HwParams::any(&pcm).unwrap();
                hwp.set_channels(2).unwrap();
                hwp.set_rate_near(fps as u32,alsa::ValueOr::Nearest).unwrap();
                hwp.set_format(Format::s16()).unwrap();
                hwp.set_access(Access::RWInterleaved).unwrap();
                hwp.set_buffer_size(1024).unwrap();
                hwp.set_period_size(256, alsa::ValueOr::Nearest).unwrap();
                pcm.hw_params(&hwp).unwrap();
            }

            {
                let hwp = pcm.hw_params_current().unwrap();
                let swp = pcm.sw_params_current().unwrap();
                let bufsize = hwp.get_buffer_size().unwrap();
                let periodsize = hwp.get_period_size().unwrap();
                swp.set_start_threshold(bufsize - periodsize).unwrap();
                swp.set_avail_min(periodsize).unwrap();
                pcm.sw_params(&swp).unwrap();
                fps = hwp.get_rate().unwrap() as f32;
            }

            // get file descriptor
            let mut fds = pcm.get().unwrap();

            // get IO interface
            let io = pcm.io_i16().unwrap();

            // start playing
            match pcm.start() {
                Err(_e) => { pcm.prepare().unwrap(); },
                _ => { },
            };

            let mut track: Option<Track> = None;
            let mut effects: Vec<Effect> = Vec::new();
            let mut track_volume: f32 = 1.0;
            let mut track_panning: f32 = 0.0;
            let mut effect_volume: f32 = 1.0;
            let mut effect_panning: f32 = 0.0;
            let mut effect_filter: Option<Filter> = None;
            let mut edit_track: Track = Track::new();
            let mut edit_synth: Synth = Synth::new();

            let mut accu: Box<[Frame]> = Box::new([Frame { l: 0.0, r: 0.0, }; 44100]);
            let mut effect_accu: Box<[Frame]> = Box::new([Frame { l: 0.0, r: 0.0, }; 44100]);
            let mut quantum: Box<[Frame]> = Box::new([Frame { l: 0.0, r: 0.0, }; 44100]);
            let mut buffer: Box<[i16]> = Box::new([0; 4 * 44100]);

            let mut running = true;
            while running {

                // flush channel
                loop {
                    match rx.try_recv() {
                        Ok(msg) => {
                            match msg {
                                AudioMsg::Terminate => { running = false; break; },
                                AudioMsg::Silence => { },  // TODO: stop everything
                                AudioMsg::Track(t) => { track = Some(t); },
                                AudioMsg::Effect(effect) => { effects.push(effect); },
                                AudioMsg::TrackVolume(volume) => { track_volume = volume; },
                                AudioMsg::TrackPanning(panning) => { track_panning = panning; },
                                AudioMsg::EffectVolume(volume) => { effect_volume = volume; },
                                AudioMsg::EffectPanning(panning) => { effect_panning = panning; },
                                AudioMsg::EffectFilter(filter) => { effect_filter = filter; },
                                AudioMsg::EditSetTrack(track) => { edit_track = track; },
                                AudioMsg::EditGetTrack => { },
                                AudioMsg::EditPlay => { },
                                AudioMsg::EditPause => { },
                                AudioMsg::EditStop => { },
                                AudioMsg::EditSpeed(_speed) => { },
                                AudioMsg::EditChannels(_n) => { },
                                AudioMsg::EditClear(_pos,_length) => { },
                                AudioMsg::EditChannelClear(_n,_pos,_length) => { },
                                AudioMsg::EditChannelVolume(_n,_volume) => { },
                                AudioMsg::EditChannelPanning(_n,_panning) => { },
                                AudioMsg::EditChannelEnabled(_n,_enabled) => { },
                                AudioMsg::EditChannelPatch(_n,_patch) => { },
                                AudioMsg::EditChannelNote(_n,_pos,_note,_velocity,_length) => { },
                                AudioMsg::EditSetPatch(patch) => { edit_synth.load(&patch); },
                                AudioMsg::EditGetPatch => { },
                                AudioMsg::EditNote(n,v) => { edit_synth.note(n,v); },
                                AudioMsg::EditRelease(n) => { edit_synth.release(n); },
                            }
                        },
                        _ => { break; },
                    }
                }

                // sleep until new data needs to be sent
                alsa::poll::poll(&mut fds,100).unwrap();

                // get number of frames to be generated
                let avail = match pcm.avail_update() {
                    Ok(n) => n as usize,
                    Err(e) => {
                        match e.errno() {
                            Some(errno) => { pcm.recover(errno as c_int,true).unwrap(); },
                            _ => { },
                        };
                        pcm.avail_update().unwrap() as usize
                    }
                };

                if avail > 0 {

                    // clear accumulator buffer (let's see if this gets compiled down to a REP STOSD or equivalent)
                    for i in 0..avail {
                        accu[i].l = 0.0;
                        accu[i].r = 0.0;
                    }

                    // render currently playing track
                    if let Some(t) = &mut track {

                        t.render(&mut quantum[0..avail]);
                        for i in 0..avail {
                            accu[i].l += 0.5 * track_volume * (1.0 + track_panning) * quantum[i].l;
                            accu[i].r += 0.5 * track_volume * (1.0 - track_panning) * quantum[i].r;
                        }
                    }

                    // render currently playing sound effects
                    for i in 0..avail {
                        effect_accu[i].l = 0.0;
                        effect_accu[i].r = 0.0;
                    }
                    for e in &effects {

                        e.render(&mut quantum[0..avail]);
                        for i in 0..avail {
                            effect_accu[i].l += 0.5 * effect_volume * (1.0 + effect_panning) * quantum[i].l;
                            effect_accu[i].r += 0.5 * effect_volume * (1.0 - effect_panning) * quantum[i].r;
                        }
                    }
                    
                    if let Some(f) = &effect_filter {
                        f.process(&mut quantum[0..avail],&effect_accu[0..avail]);
                        for i in 0..avail {
                            accu[i].l += quantum[i].l;
                            accu[i].r += quantum[i].r;
                        }
                    }
                    else {
                        for i in 0..avail {
                            accu[i].l += effect_accu[i].l;
                            accu[i].r += effect_accu[i].r;
                        }
                    }

                    // render currently playing edit track
                    edit_track.render(&mut quantum[0..avail]);
                    for i in 0..avail {
                        accu[i].l += quantum[i].l;
                        accu[i].r += quantum[i].r;
                    }

                    // render realtime notes on current synth
                    edit_synth.render(fps,&mut quantum[0..avail]);
                    for i in 0..avail {
                        accu[i].l += quantum[i].l;
                        accu[i].r += quantum[i].r;
                    }

                    // convert to interleaved buffer
                    for i in 0..avail {
                        buffer[i * 2] = (clamp(accu[i].l,-1.0,1.0) * 32767.0) as i16;
                        buffer[i * 2 + 1] = (clamp(accu[i].r,-1.0,1.0) * 32767.0) as i16;
                    }

                    // and write
                    match io.writei(&buffer[0..avail * 2]) {
                        Ok(_n) => { },
                        Err(e) => { println!("Audio: write error {}",e); },
                    };
                }
            }
        }));

        Audio {
            mixer_thread: mixer_thread,
            tx: tx,
            rx_return: rx_return,
        }
    }

    /// Terminate the audio output.
    pub fn terminate(&self) {
        self.tx.send(AudioMsg::Terminate).unwrap();
    }

    /// Stop playing, if anything is playing.
    pub fn silence(&self) {
        self.tx.send(AudioMsg::Silence).unwrap();
    }

    /// Start playing a music track. This replaces an already playing track.
    /// # Arguments.
    /// * `track` - The track.
    pub fn track(&self,track: Track) {
        self.tx.send(AudioMsg::Track(track)).unwrap();
    }

    /// Start playing a sound effect. This plays on top of other sound effects.
    /// # Arguments.
    /// * `effect` - The effect.
    pub fn effect(&self,effect: Effect) {
        self.tx.send(AudioMsg::Effect(effect)).unwrap();
    }

    /// Set the music track volume.
    /// # Arguments
    /// * `volume` - The volume (0..1).
    pub fn track_volume(&self,volume: f32) {
        self.tx.send(AudioMsg::TrackVolume(volume)).unwrap();
    }

    /// Set the music track panning.
    /// # Arguments
    /// * `panning` - The panning (-1..1).
    pub fn track_panning(&self,panning: f32) {
        self.tx.send(AudioMsg::TrackPanning(panning)).unwrap();
    }

    /// Set the sound effect volume.
    /// # Arguments
    /// * `volume` - The volume (0..1).
    pub fn effect_volume(&self,volume: f32) {
        self.tx.send(AudioMsg::EffectVolume(volume)).unwrap();
    }

    /// Set the sound effect panning.
    /// # Arguments
    /// * `panning` - The panning (-1..1).
    pub fn effect_panning(&self,panning: f32) {
        self.tx.send(AudioMsg::EffectPanning(panning)).unwrap();
    }

    /// Specify a filter for the sound effects.
    /// # Arguments
    /// * `filter` - The filter, or None.
    pub fn effect_filter(&self,filter: Option<Filter>) {
        self.tx.send(AudioMsg::EffectFilter(filter)).unwrap();
    }

    /// Editing API: Load music track to edit.
    /// # Arguments
    /// * `track` - The track.
    pub fn edit_set_track(&self,track: Track) {
        self.tx.send(AudioMsg::EditSetTrack(track)).unwrap();
    }

    /// Editing API: Get copy of currently editing track.
    /// # Returns
    /// Copy of the currently editing track.
    pub fn edit_get_track(&self) {
        self.tx.send(AudioMsg::EditGetTrack).unwrap();
        // TODO: wait for Track message from rx_return
    }

    /// Editing API: Start playing the currently editing track.
    pub fn edit_play(&self) {
        self.tx.send(AudioMsg::EditPlay).unwrap();
    }

    /// Editing API: Pause/resume the currently editing track.
    pub fn edit_pause(&self) {
        self.tx.send(AudioMsg::EditPause).unwrap();
    }

    /// Editing API: Stop playing the currently editing track.
    pub fn edit_stop(&self) {
        self.tx.send(AudioMsg::EditStop).unwrap();
    }

    /// Editing API: Set speed.
    /// # Arguments
    /// * `bpm` - Speed (bpm).
    pub fn edit_speed(&self,bpm: u32) {
        self.tx.send(AudioMsg::EditSpeed(bpm)).unwrap();
    }

    /// Editing API: Set number of channels.
    /// # Arguments
    /// * `n` - Number of channels.
    pub fn edit_channels(&self,n: u32) {
        self.tx.send(AudioMsg::EditChannels(n)).unwrap();
    }

    /// Editing API: Clear all channels.
    /// # Arguments
    /// * `pos` - From position.
    /// * `length` - Length to clear.
    pub fn edit_clear(&self,pos: u64,length: u64) {
        self.tx.send(AudioMsg::EditClear(pos,length)).unwrap();
    }

    /// Editing API: Clear channel.
    /// # Arguments
    /// * `n` - Channel to clear.
    /// * `pos` - From position.
    /// * `length` - Length to clear.
    pub fn edit_channel_clear(&self,n: u32,pos: u64,length: u64) {
        self.tx.send(AudioMsg::EditChannelClear(n,pos,length)).unwrap();
    }

    /// Editing API: Set volume for specific channel.
    /// # Arguments
    /// * `n` - Channel to set volume for.
    /// * `volume` - Volume (0..1).
    pub fn edit_channel_volume(&self,n: u32,volume: f32) {
        self.tx.send(AudioMsg::EditChannelVolume(n,volume)).unwrap();
    }

    /// Editing API: Set panning for specific channel.
    /// # Arguments
    /// * `n` - Channel to set panning for.
    /// * `panning` - Panning (-1..1).
    pub fn edit_channel_panning(&self,n: u32,panning: f32) {
        self.tx.send(AudioMsg::EditChannelPanning(n,panning)).unwrap();
    }

    /// Editing API: Set enabled for specific channel.
    /// # Arguments
    /// * `n` - Channel to set enabled for.
    /// * `enabled` - State (False = muted, True = normal).
    pub fn edit_channel_enabled(&self,n: u32,enabled: bool) {
        self.tx.send(AudioMsg::EditChannelEnabled(n,enabled)).unwrap();
    }

    /// Editing API: Set synth patch to use for the channel.
    /// # Arguments
    /// * `n` - Channel to set synth for.
    /// * `patch` - Patch to use.
    pub fn edit_channel_patch(&self,n: u32,patch: Patch) {
        self.tx.send(AudioMsg::EditChannelPatch(n,patch)).unwrap();
    }

    /// Editing API: Set note for a channel.
    /// # Arguments
    /// * `n` - Channel to set note for.
    /// * `pos` - Where to set the note.
    /// * `note` - Which note to set.
    /// * `velocity` - Key velocity for the note.
    /// * `length` - Length of the note.
    pub fn edit_channel_note(&self,n: u32, pos: u64, note: usize, velocity: f32,length: u64) {
        self.tx.send(AudioMsg::EditChannelNote(n,pos,note,velocity,length)).unwrap();
    }

    /// Editing API: Set current live patch.
    /// * `patch` - New current patch.
    pub fn edit_set_patch(&self,patch: Patch) {
        self.tx.send(AudioMsg::EditSetPatch(patch)).unwrap();
    }

    /// Editing API: Get copy of current live patch.
    pub fn edit_get_patch(&self) {
        self.tx.send(AudioMsg::EditGetPatch).unwrap();
        // TODO: wait for Patch message from rx_return
    }

    /// Editing API: Play live note.
    /// # Arguments
    /// * `note` - Note to play.
    /// * `velocity` - Key velocity to play at.
    pub fn edit_note(&self,note: usize, velocity: f32) {
        self.tx.send(AudioMsg::EditNote(note,velocity)).unwrap();
    }

    /// Editing API: Stop live note.
    /// # Arguments
    /// * `note` - Which note to release.
    pub fn edit_release(&self,note: usize) {
        self.tx.send(AudioMsg::EditRelease(note)).unwrap();
    }
}

impl Drop for Audio {

    /// Terminate and destroy the audio output.
    fn drop(&mut self) {
        self.tx.send(AudioMsg::Terminate).unwrap();
        if let Some(handle) = self.mixer_thread.take() {
            handle.join().unwrap();
        }
    }
}
*/