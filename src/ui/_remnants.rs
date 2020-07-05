fn build_color_shader() -> Shader {
    let vs = r#"
        #version 420 core

        uniform vec2 scale;

        layout(location = 0) in vec4 p;

        void main() {
            gl_Position = vec4(-1.0 + 2.0 * scale.x * p.x,-1.0 + 2.0 * scale.y * p.y,0.0,1.0);
        }
    "#;

    let fs = r#"
        #version 420 core

        uniform vec4 color;

        out vec4 frag_color;

        void main() {
            frag_color = color;
        }
    "#;

    UI::_create_shader(vs,None,fs).expect("what?")
}

fn build_rect_shader() -> Shader {
    let vs = r#"
        #version 420 core

        uniform vec2 scale;
        uniform vec2 origin;
        uniform vec2 size;

        layout(location = 0) in vec4 p;

        void main() {
            gl_Position = vec4(-1.0 + 2.0 * scale.x * (origin.x + p.x * size.x),-1.0 + 2.0 * scale.y * (origin.y + p.y * size.y),0.0,1.0);
        }
    "#;

    let fs = r#"
        #version 420 core

        uniform vec4 color;

        out vec4 frag_color;

        void main() {
            frag_color = color;
        }
    "#;

    UI::_create_shader(vs,None,fs).expect("what?")
}


fn build_quad_vb() -> VertexBuffer<Vec2<f32>> {
    let mut vertices: Vec<Vec2<f32>> = Vec::new();
    vertices.push(vec2!(0.0,0.0));
    vertices.push(vec2!(1.0,0.0));
    vertices.push(vec2!(1.0,1.0));
    vertices.push(vec2!(0.0,1.0));
    UI::_create_vertexbuffer(vertices).expect("what?")
}


let color_shader = build_color_shader();
let rect_shader = build_rect_shader();
let quad_vb = build_quad_vb();


color_shader: color_shader,
rect_shader: rect_shader,
quad_vb: quad_vb,


pub(crate) fn create_gc(&self) -> GC {
    GC {
        opengl: &self,
        size: Cell::new(vec2!(1,1)),
        scale: Cell::new(SCREEN),
        color: Cell::new(vec4!(1.0,1.0,0.0,1.0)),
    }
}


pub fn set_scale(&self,scale: Vec2<f32>) {  // set pixels per GU
    self.scale.set(vec2!(scale.x * SCREEN.x,scale.y * SCREEN.y));
}

pub fn get_scale(&self) -> Vec2<f32> {
    self.scale.get()
}

pub fn set_window_size(&self,size: Vec2<usize>) {  // set size of the window in pixels
    self.size.set(size);
}

pub fn get_space(&self) -> Vec2<f32> {  // get maximum GU coordinates
    let scale = self.scale.get();
    let size = self.size.get();
    vec2!((size.x as f32) / scale.x,(size.y as f32) / scale.y)
}


pub fn bind_color_shader(&self) {
    unsafe { gl::UseProgram(self.opengl.color_shader.sp); }
    self.sp.set(self.opengl.color_shader.sp);
}

pub fn bind_rect_shader(&self) {
    unsafe { gl::UseProgram(self.opengl.rect_shader.sp); }
    self.sp.set(self.opengl.rect_shader.sp);
}


pub fn draw_rect<T>(&self,r: Rect<f32>,color: T) where Vec4<f32>: From<T> {
    self.bind_vertexbuffer(&self.opengl.quad_vb);
    self.bind_rect_shader();
    let scale = self.scale.get();
    let size = self.size.get();
    self.set_uniform("scale",vec2!(scale.x / (size.x as f32),scale.y / (size.y as f32)));
    self.set_uniform("origin",r.o);
    self.set_uniform("size",r.s);
    self.set_uniform("color",Vec4::<f32>::from(color));
    self.draw_triangle_fan(4);
    self.unbind_vertexbuffer();
    self.unbind_shader();
}

// at Paint event:
self.set_window_size(size);
let space = self.get_space();
