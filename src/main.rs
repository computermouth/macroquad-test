mod raylib;
use raylib::*;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 500,
        window_height: 700,
        fullscreen: false,
        ..Default::default()
    }
}

#[derive(Copy, Clone)]
struct Wall {
    p1: Vector2,
    p2: Vector2,
}

#[macroquad::main(conf)]
async fn main() {
    set_cursor_grab(false);
    // show_mouse(false);

    #[rustfmt::skip]
    let walls = vec![
        Wall { // top
            p1: Vector2{ x:  50., y: 50. },
            p2: Vector2{ x: 450., y: 50. },
        },
        Wall { // left
            p1: Vector2{ x:  50., y:  50. },
            p2: Vector2{ x:  50., y: 450. },
        },
        Wall { // right
            p1: Vector2{ x: 450., y: 450. },
            p2: Vector2{ x: 450., y:  50. },
        },
        Wall { // bottom
            p1: Vector2{ x: 450., y: 450. },
            p2: Vector2{ x:  50., y: 450. },
        },
        Wall { // TLcorner
            p1: Vector2{ x: 200., y: 200. },
            p2: Vector2{ x: 200., y:  50. },
        },
        Wall { // TLcorner
            p1: Vector2{ x: 200., y: 200. },
            p2: Vector2{ x:  50., y: 200. },
        },
        Wall { // BLdiag
            p1: Vector2{ x:   0., y: 300. },
            p2: Vector2{ x: 200., y: 500. },
        },
        Wall { // |_| botto
            p1: Vector2{ x: 400., y: 400. },
            p2: Vector2{ x: 500., y: 400. },
        },
        Wall { // // |_| to
            p1: Vector2{ x: 400., y: 390. },
            p2: Vector2{ x: 500., y: 390. },
        },
        Wall { // |_| left
            p1: Vector2{ x: 400., y: 390. },
            p2: Vector2{ x: 400., y: 400. },
        },
    ];

    let mut player_pos = Vector2 { x: 250., y: 250. };
    let mut rotation = 0.;
    let mut velocity = Vector2 { x: 0., y: 0. };
    let player_radius = 10.;

    loop {
        let delta = get_frame_time();

        #[rustfmt::skip]
        let key_r = match is_key_down(KeyCode::Right) { true => 1., false => 0.};
        #[rustfmt::skip]
        let key_l = match is_key_down(KeyCode::Left ) { true => 1., false => 0.};
        #[rustfmt::skip]
        let key_w = match is_key_down(KeyCode::W    ) { true => 1., false => 0.};
        #[rustfmt::skip]
        let key_a = match is_key_down(KeyCode::A    ) { true => 1., false => 0.};
        #[rustfmt::skip]
        let key_s = match is_key_down(KeyCode::S    ) { true => 1., false => 0.};
        #[rustfmt::skip]
        let key_d = match is_key_down(KeyCode::D    ) { true => 1., false => 0.};

        rotation = (rotation + 4. * (key_r - key_l) * delta) % (3.14159 * 2.);
        let acceleration = vector2_rotate(
            Vector2 {
                x: 4. * (key_a - key_d),
                y: 4. * (key_w - key_s),
            },
            rotation,
        );

        // generate velocity with friction
        let delta_friction = 2. * delta;
        let delta_acceleration = vector2_scale(acceleration, delta);
        // let delta_acceleration = Vec2::ZERO;
        let friction = vector2_scale(velocity, delta_friction);
        // let friction = Vec2::ZERO;
        velocity = vector2_add(velocity, vector2_subtract(delta_acceleration, friction));

        clear_background(DARKGRAY);

        // draw walls
        for wall in &walls {
            draw_line(wall.p1.x, wall.p1.y, wall.p2.x, wall.p2.y, 2., WHITE);
            let closest = closest_point_on_line_segment(*wall, player_pos);
            draw_circle(closest.x, closest.y, 5., GRAY);
        }

        let last_step = velocity;

        let step_count = 16;
        let step_len = vector2_scale(velocity, 1. / step_count as f32);
        for _ in 0..step_count {
            // add a step to player pos
            let mut local_step = step_len;
            let mut local_last = player_pos;
            let mut local_pos = vector2_add(player_pos, local_step);
            let mut iter = 0;

            'out: loop {
                if iter == 5 {
                    local_pos = local_last;
                    break 'out;
                } else if let Some((nearest, wall)) =
                    find_nearest_collision(local_pos, player_radius, &walls)
                {
                    draw_line(wall.p1.x, wall.p1.y, wall.p2.x, wall.p2.y, 4., PINK);
                    draw_circle(nearest.x, nearest.y, 5., RED);

                    let collision_normal = vector2_normalize(vector2_subtract(local_pos, nearest));
                    let closest_distance = vector2_distance(local_pos, nearest);

                    // Move the circle to just touch the wall
                    let penetration_depth = player_radius - closest_distance;
                    local_pos = vector2_add(
                        local_pos,
                        vector2_scale(collision_normal, penetration_depth),
                    );

                    // Calculate new velocity to slide along the wall
                    let mut projected_velocity = vector2_subtract(
                        velocity,
                        vector2_scale(
                            collision_normal,
                            vector2_dot_product(velocity, collision_normal),
                        ),
                    );

                    if vector2_length(projected_velocity) < VCLOSE {
                        projected_velocity = Vector2 { x: 0., y: 0. };
                    }

                    // Adjust `local_step` to account for the distance already traveled
                    let remaining_distance = vector2_length(step_len) - closest_distance;
                    local_step =
                        vector2_scale(vector2_normalize(projected_velocity), remaining_distance);
                    local_last = local_pos;
                    iter += 1;
                } else {
                    local_pos = local_last;
                    break 'out;
                }
            }

            player_pos = local_pos;
        }

        // draw player
        player_pos = vector2_add(player_pos, velocity);
        draw_circle(player_pos.x, player_pos.y, player_radius, BLUE);
        // draw player look dir
        let line_end = vector2_add(
            vector2_rotate(Vector2 { x: 0., y: 60. }, rotation),
            player_pos,
        );
        draw_line(
            player_pos.x,
            player_pos.y,
            line_end.x,
            line_end.y,
            1.,
            BLACK,
        );

        draw_text(
            format!("FPS: {}", 119).as_str(),
            10.0,
            500. + 30.0 * 1.,
            30.0,
            WHITE,
        );

        draw_text(
            format!("ROT: {:04.1}", rotation).as_str(),
            10.0,
            500. + 30.0 * 2.,
            30.0,
            RED,
        );

        draw_text(
            format!(
                "POS: {{ x: {:04.1}  y: {:04.1} }}",
                player_pos.x, player_pos.y
            )
            .as_str(),
            10.0,
            500. + 30.0 * 3.,
            30.0,
            GREEN,
        );

        draw_text(
            format!(
                "ACC: {{ x: {:04.1}  y: {:04.1} }}",
                acceleration.x, acceleration.y
            )
            .as_str(),
            10.0,
            500. + 30.0 * 4.,
            30.0,
            PINK,
        );

        draw_text(
            format!("VEL: {{ x: {:04.1}  y: {:04.1} }}", velocity.x, velocity.y).as_str(),
            10.0,
            500. + 30.0 * 5.,
            30.0,
            ORANGE,
        );

        draw_text(
            format!(
                "SLD: {{ x: {:04.1}  y: {:04.1} }}",
                last_step.x, last_step.y
            )
            .as_str(),
            10.0,
            500. + 30.0 * 6.,
            30.0,
            SKYBLUE,
        );

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

const VCLOSE: f32 = 0.001;

fn closest_point_on_line_segment(wall: Wall, point: Vector2) -> Vector2 {
    let line_vector = vector2_subtract(wall.p2, wall.p1);
    let line_length = vector2_length(line_vector);
    let line_direction = vector2_normalize(line_vector);

    let t = (vector2_dot_product(vector2_subtract(point, wall.p1), line_direction) / line_length)
        .clamp(0., 1.);

    vector2_add(wall.p1, vector2_scale(line_vector, t))
}

fn find_nearest_collision(
    circle_pos: Vector2,
    circle_radius: f32,
    walls: &Vec<Wall>,
) -> Option<(Vector2, Wall)> {
    let mut nearest = None;

    for wall in walls {
        let closest_point = closest_point_on_line_segment(*wall, circle_pos);
        let distance_to_circle = vector2_distance(circle_pos, closest_point);

        if distance_to_circle <= circle_radius + VCLOSE {
            match nearest {
                None => nearest = Some((closest_point, *wall)),
                Some((old, _)) => {
                    if distance_to_circle < vector2_distance(circle_pos, old) {
                        nearest = Some((closest_point, *wall))
                    }
                }
            }
        }
    }

    // scale it back a smidge for floating point nonsense
    if let Some((n, wall)) = nearest {
        let ndir = vector2_normalize(vector2_subtract(circle_pos, n));
        let new_nearest = vector2_add(n, vector2_scale(ndir, VCLOSE));
        nearest = Some((new_nearest, wall));
    }

    nearest
}

// fn collide_and_slide(circle_pos: Vector2, circle_radius: f32, velocity: Vector2, walls: Vec<Wall>) -> (Vector2, Vector2) {

// }

// function detect_and_slide(circle_pos, circle_radius, velocity, wall_segments):

//     // Step 2: Handle the closest collision if there is one
//     if closest_collision is not null:
//         collision_normal = closest_collision.normal
//         collision_point = closest_collision.point

//         // Move the circle to just touch the wall
//         penetration_depth = circle_radius - closest_distance
//         circle_pos = circle_pos + (collision_normal * penetration_depth)

//         // Calculate new velocity to slide along the wall
//         projected_velocity = velocity - dot(velocity, collision_normal) * collision_normal

//         // If the projected_velocity still points into the wall, zero it out
//         if dot(projected_velocity, collision_normal) > 0:
//             projected_velocity = Vector2(0, 0)

//         return circle_pos, projected_velocity

//     // No collision detected, proceed as normal
//     return circle_pos + velocity, velocity
