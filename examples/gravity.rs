use craig_iter::{IntoNextWithRestIterator, LendingIterator};
use rand::Rng;
use macroquad::prelude as mq;

const G: f64 = 6.674e-11;
const SOLAR_MASS: f64 = 1.989e30;
const SOLAR_RADIUS: f64 = 6.957e8;
const AU: f64 = 1.496e11;
const UNIVERSE_SIZE: f64 = 10.0*AU;
const SCREEN_SIZE : f64 = 600.0;
const DRAW_SCALE: f64 = SCREEN_SIZE / UNIVERSE_SIZE;
const SECS_IN_YEAR: f64 = 3.154e+7;
const TIME_SCALE: f64 = SECS_IN_YEAR / 100.0;

struct Body {
    id: usize,
    p_vec: (f64, f64),
    u_vec: (f64, f64),
    a_vec: (f64, f64),
    f_vec: (f64, f64),
    m: f64,
}

impl Body {
    fn new_random() -> Self {
        Self {
            id: rand::thread_rng().gen(),
            p_vec: (
                rand::thread_rng().gen_range(0.0..UNIVERSE_SIZE),
                rand::thread_rng().gen_range(0.0..UNIVERSE_SIZE)
            ),
            u_vec: (
                0.0,
                0.0,
            ),
            a_vec: (
                0.0,
                0.0,
            ),
            f_vec: (
                0.0,
                0.0,
            ),
            m: SOLAR_MASS,
         }
    }
    #[allow(dead_code)]
    fn new(
        p_vec: (f64, f64),
        u_vec: (f64, f64),
        a_vec: (f64, f64),
        f_vec: (f64, f64),
        m: f64) -> Self {
        Self {
            id: rand::thread_rng().gen(),
            p_vec,
            u_vec,
            a_vec,
            f_vec,
            m
         }
    }
}

enum Command {
    Delete{
        id: usize,
    },
    Create{
        p_vec: (f64, f64),
        mass: f64,
    }
}

fn main() {
    macroquad::Window::from_config(
        mq::Conf {
            sample_count: 4,
            window_title: "Gravity".to_owned(),
            high_dpi: true,
            window_width: SCREEN_SIZE as i32,
            window_height: SCREEN_SIZE as i32,
            ..Default::default()
        },
        amain(),
    );
}

async fn amain() {

    let mut bodies = Vec::new();
    for _ in 0..100 {
        bodies.push(Body::new_random());
    }
    loop {
        mq::clear_background(mq::BLACK);
        // //Update bodies
        let mut commands = Vec::new();
        let dt = mq::get_frame_time() as f64 * TIME_SCALE;
        let mut iter = bodies.iter_with_rest();
        while let Some((this, others)) = iter.next() {
            //if this has been marked for deletion, skip it
            if commands.iter().any(|c| match c {
                Command::Delete{id} => *id == this.id,
                _ => false,
            }) {continue;}
            
            let mut f_vec = (0.0, 0.0);
            for other in others {
                let dx = other.p_vec.0 - this.p_vec.0;
                let dy = other.p_vec.1 - this.p_vec.1;
                let r = (dx * dx + dy * dy).sqrt();
                if r == 0.0 {continue};
                let f = G * this.m * other.m / (r * r);
                let fx = f * dx / r;
                let fy = f * dy / r;
                f_vec.0 += fx;
                f_vec.1 += fy;
                if r < SOLAR_RADIUS {
                    commands.push(Command::Delete{id: this.id});
                    commands.push(Command::Delete{id: other.id});
                    commands.push(Command::Create{
                        p_vec: this.p_vec,
                        mass: this.m + other.m,
                    });
                }
            }
            this.f_vec.0 = f_vec.0;
            this.f_vec.1 = f_vec.1;
            this.a_vec.0 = f_vec.0 / this.m;
            this.a_vec.1 = f_vec.1 / this.m;
            this.u_vec.0 += this.a_vec.0 * dt ;
            this.u_vec.1 += this.a_vec.1 * dt ;
            this.p_vec.0 += this.u_vec.0 * dt;
            this.p_vec.1 += this.u_vec.1 * dt;
        }
        // Draw bodies
        for particle in bodies.iter() {
            mq::draw_circle(
                particle.p_vec.0 as f32 * DRAW_SCALE as f32,
                particle.p_vec.1 as f32 * DRAW_SCALE as f32,
                (SOLAR_RADIUS * DRAW_SCALE * 2.0) as f32,
                mq::WHITE,
            );
        }
        //process commands
        for c in commands.iter() {
            match c {
                Command::Delete{id} => {
                    bodies.retain(|b| b.id != *id);
                },
                Command::Create{p_vec, mass} => {
                    bodies.push(Body::new(
                        *p_vec,
                        (0.0, 0.0),
                        (0.0, 0.0),
                        (0.0, 0.0),
                        *mass,
                    ));
                }
            }
        }
        commands.clear();
        if mq::is_key_pressed(mq::KeyCode::Space) {
            bodies.clear();
            for _ in 0..100 {
                bodies.push(Body::new_random());
            }
        }
        mq::next_frame().await;
    };
}