use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[derive(Debug)]
struct Entity {
    pos: Vec3,
    f_c: Color,
    l_c: Color,
    dir: Vec3,
}

impl Entity {
    fn new(pos: Option<Vec3>, dir: Option<Vec3>) -> Entity {
        let (fc, lc) = match rand::gen_range(0, 14) {
            0 => (DARKBLUE, BLUE),
            1 => (DARKBLUE, VIOLET),
            2 => (DARKBLUE, SKYBLUE),
            3 => (DARKBROWN, BROWN),
            4 => (DARKGRAY, GRAY),
            5 => (DARKGREEN, GREEN),
            6 => (DARKGREEN, GOLD),
            7 => (DARKGREEN, LIME),
            8 => (DARKGREEN, YELLOW),
            9 => (DARKPURPLE, MAROON),
            10 => (DARKPURPLE, PURPLE),
            11 => (DARKPURPLE, MAGENTA),
            12 => (DARKPURPLE, RED),
            13 => (DARKPURPLE, PINK),
            _ => unreachable!(),
        };

        let e = Entity {
            pos: pos.map_or_else(
                || {
                    vec3(
                        rand::gen_range(2., 7.),
                        rand::gen_range(2., 7.),
                        rand::gen_range(2., 7.),
                    )
                },
                |v| v,
            ),
            f_c: fc,
            l_c: lc,
            dir: dir.map_or_else(
                || {
                    vec3(
                        rand::gen_range(0.2, 0.7),
                        rand::gen_range(0.2, 0.7),
                        rand::gen_range(0.2, 0.7),
                    )
                },
                |v| v,
            ),
        };

        // println!("ent: {:?}", e);

        e
    }

    fn update(&mut self, delta: f32) -> Option<Entity> {
        let mut rc = None;

        self.pos = self.pos + (self.dir * delta);

        let collided_pos = self.pos.clamp(vec3(0.5, 0.5, 0.5), vec3(9.5, 9.5, 9.5));

        if self.pos != collided_pos {

            let mut coll_dir = (self.pos - collided_pos).abs().ceil() * -1.0;
            if coll_dir.x != -1.0 {
                coll_dir.x = 1.0;
            }
            if coll_dir.y != -1.0 {
                coll_dir.y = 1.0;
            }
            if coll_dir.z != -1.0 {
                coll_dir.z = 1.0;
            }

            // println!("collided_pos: {:?}\nself.pos: {:?}", collided_pos, self.pos);
            // println!("coll_dir: {:?}\nself_dir: {:?}", coll_dir, self.dir);

            self.dir *= coll_dir;
            self.pos = collided_pos + self.dir * delta;

            // let new_dir = vec3(
            //     rand::gen_range(coll_dir.x * 0.2, coll_dir.x * 0.7),
            //     rand::gen_range(coll_dir.y * 0.2, coll_dir.y * 0.7),
            //     rand::gen_range(coll_dir.z * 0.2, coll_dir.z * 0.7),
            // );
            // println!("new_dir: {:?}", new_dir);

            rc = Some(Entity::new(None, None));
        }

        self.draw();
        
        rc
    }

    fn draw(&self){
        draw_cube(self.pos, vec3(1., 1., 1.), None, self.f_c);
        draw_cube_wires(self.pos, vec3(1., 1., 1.), self.l_c);
    }
}

#[macroquad::main(conf)]
async fn main() {
    // let mut x = 0.0;
    // let mut switch = false;
    // let bounds = 8.0;

    // let world_up = vec3(0.0, 1.0, 0.0);
    // let mut yaw: f32 = -2.25;
    // let mut pitch: f32 = -0.75;

    // let mut front = vec3(
    //     yaw.cos() * pitch.cos(),
    //     pitch.sin(),
    //     yaw.sin() * pitch.cos(),
    // )
    // .normalize();
    // let mut right = front.cross(world_up).normalize();
    // let mut up;

    // let mut position = vec3(15.0, 15.0, 15.0);

    // let mut grabbed = true;
    set_cursor_grab(false);
    show_mouse(false);

    // println!(
    //     "thhing: {:?}",
    //     vec3(0.0, 2.0, 10.5).abs().normalize().ceil()
    // );

    let mut entities = vec![Entity::new(None, None)];

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        // if is_key_pressed(KeyCode::Tab) {
        //     grabbed = !grabbed;
        //     set_cursor_grab(grabbed);
        //     show_mouse(!grabbed);
        // }

        // if is_key_down(KeyCode::Up) {
        //     position += front * MOVE_SPEED;
        // }
        // if is_key_down(KeyCode::Down) {
        //     position -= front * MOVE_SPEED;
        // }
        // if is_key_down(KeyCode::Left) {
        //     position -= right * MOVE_SPEED;
        // }
        // if is_key_down(KeyCode::Right) {
        //     position += right * MOVE_SPEED;
        // }

        // let mouse_delta = mouse_delta_position();
        // if grabbed {
        //     yaw += mouse_delta.x * delta * LOOK_SPEED * -500.0;
        //     pitch += mouse_delta.y * delta * -LOOK_SPEED * -500.0;

        //     pitch = if pitch > 1.5 { 1.5 } else { pitch };
        //     pitch = if pitch < -1.5 { -1.5 } else { pitch };
        // }

        // front = vec3(
        //     yaw.cos() * pitch.cos(),
        //     pitch.sin(),
        //     yaw.sin() * pitch.cos(),
        // )
        // .normalize();

        // right = front.cross(world_up).normalize();
        // up = right.cross(front).normalize();

        // x += if switch { 0.04 } else { -0.04 };
        // if x >= bounds || x <= -bounds {
        //     switch = !switch;
        // }

        clear_background(LIGHTGRAY);

        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(22., 15., 18.),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(5.,0.,5.),
            ..Default::default()
        });

        let mut new_e = Vec::new();
        for i in &mut entities {
            if let Some(e) = i.update(delta) {
                new_e.push(e);
            }
        }

        entities.append(&mut new_e);

        draw_cube(vec3(5., -0.5, 5.), vec3(10., 1., 10.), None, WHITE);
        draw_cube_wires(vec3(5., 5., 5.), vec3(10., 10., 10.), GRAY);

        // Back to screen space, render some text
        set_default_camera();
        draw_text("--bounce--", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            48.0 + 18.0,
            30.0,
            BLACK,
        );

        draw_text(
            format!("Cubes: {}", entities.len()).as_str(),
            10.0,
            48.0 + 42.0,
            30.0,
            BLACK,
        );


        next_frame().await
    }
}
