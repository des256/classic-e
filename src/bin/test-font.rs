// E - Font test
// Desmond Germans, 2020

use e::UI;
use e::Event;
use e::isize_2;
use e::isize_r;
use std::rc::Rc;
use std::cell::RefCell;
use e::VertexBuffer;
use e::Shader;
use e::f32_2;
use e::Texture2D;
use e::ARGB8;
use std::fs::File;
use std::io::prelude::*;
use e::decode;
use e::Graphics;

struct App {
    running: bool,
    vertexbuffer: VertexBuffer<f32_2>,
    shader: Shader,
    texture: Texture2D<ARGB8>,
}

fn handler(event: Event,app: &mut App) {
    match event {
        Event::Paint(graphics,_) => {
            graphics.clear(0.0,0.0,0.0,1.0);
            graphics.bind_vertexbuffer(&app.vertexbuffer);
            graphics.bind_shader(&app.shader);
            graphics.bind_texture2d(0,&app.texture);
            graphics.set_uniform("font_texture",0);
            graphics.draw_triangle_fan(4);
        },
        Event::Close => {
            app.running = false;
        },
        _ => { },
    }
}

fn load_texture(graphics: &Graphics,name: &str) -> Texture2D<ARGB8> {
    let mut file = File::open(name).expect("cannot open file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("unable to read file");
    let image = decode::<ARGB8>(&buffer).expect("unable to decode");
    let texture = graphics.create_texture2d::<ARGB8>(&image).expect("what?");
    texture
}

fn main() {
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(_) => { panic!("Cannot open UI."); },
    };

    let vertexbuffer = ui.graphics().create_vertexbuffer(vec![
        f32_2 { x: 0.0,y: 0.0, },
        f32_2 { x: 1.0,y: 0.0, },
        f32_2 { x: 1.0,y: 1.0, },
        f32_2 { x: 0.0,y: 1.0, },
    ]).expect("what?");

    let vs = r#"
        #version 420 core

        layout(location = 0) in vec2 p;

        out vec2 tc;

        void main() {
            tc = vec2(p.x,1.0 - p.y);
            gl_Position = vec4(-1.0 + 2.0 * p.x,-1.0 + 2.0 * p.y,0.0,1.0);
        }
    "#;

    let fs = r#"
        #version 420 core

        uniform sampler2D font_texture;

        in vec2 tc;

        out vec4 frag_color;

        float median(float r,float g,float b) {
            return max(min(r,g),min(max(r,g),b));
        }

        void main() {
            vec3 t = texture(font_texture,tc).rgb;
            vec2 unit = (4.0 / textureSize(font_texture,0)).xy;
            float dist = median(t.r,t.g,t.b) - 0.5;
            dist *= dot(unit,0.5 / fwidth(tc));
            float cov = clamp(dist + 0.5,0.0,1.0);
            frag_color = mix(vec4(0.0,0.0,0.0,1.0),vec4(0.0,0.5,1.0,1.0),cov);
        }
    "#;

    let shader = ui.graphics().create_shader(vs,None,fs).expect("what?");

    let texture = load_texture(ui.graphics(),"arial.bmp");
    let app = Rc::new(RefCell::new(App {
        running: true,
        vertexbuffer: vertexbuffer,
        shader: shader,
        texture: texture,
    }));

    let cloned_app = app.clone();
    ui.create_window(
        &isize_r::new(isize_2::new(50,50),isize_2::new(640,360)),
        "Test Window",
        move |event| {
            let mut app = cloned_app.borrow_mut();
            handler(event,&mut *app);
        }
    );

    while app.borrow().running {
        ui.wait();
        ui.pump();
    }
}