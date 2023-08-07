use craig_iter::{IntoNextWithRestIterator, LendingIterator};
use rand::Rng;
use macroquad::prelude as mq;

const G: f64 = 6.674e-11;
const SOLAR_MASS: f64 = 1.989e30; 
const AU: f64 = 1.496e11;
const _EARTH_ORBIT_SPEED: f64 = 2.978e4;
const _EARTH_MASS: f64 = 5.972e24;
const UNIVERSE_SIZE: f64 = 10.0*AU;
const SCREEN_SIZE : f64 = 600.0; 
const DRAW_SCALE: f64 = SCREEN_SIZE / UNIVERSE_SIZE;
const SECS_IN_YEAR: f64 = 3.154e+7;
const TIME_SCALE: f64 = SECS_IN_YEAR / 100.0;

struct Particle {
    p_vec: (f64, f64),
    u_vec: (f64, f64),
    a_vec: (f64, f64),
    f_vec: (f64, f64),
    m: f64,
}

impl Particle {
    fn new_random() -> Self {
        Self {
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
            m: SOLAR_MASS
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
            p_vec,
            u_vec,
            a_vec,
            f_vec,
            m
         }
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
    //create 1000 suns
    for _ in 0..1000 {
        bodies.push(Particle::new_random());
    }

    loop {
        mq::clear_background(mq::BLACK);
        // //Update particles
        let dt = mq::get_frame_time() as f64 * TIME_SCALE;
        let mut iter = bodies.iter_with_rest();
        while let Some((particle, other_particles)) = iter.next() {
            let mut f_vec = (0.0, 0.0);
            for other_particle in other_particles {
                let dx = other_particle.p_vec.0 - particle.p_vec.0;
                let dy = other_particle.p_vec.1 - particle.p_vec.1;
                let r = (dx * dx + dy * dy).sqrt();
                let f = G * particle.m * other_particle.m / (r * r);
                let fx = f * dx / r;
                let fy = f * dy / r;
                f_vec.0 += fx;
                f_vec.1 += fy;
            }
            particle.f_vec.0 = f_vec.0;
            particle.f_vec.1 = f_vec.1;
            particle.a_vec.0 = f_vec.0 / particle.m;
            particle.a_vec.1 = f_vec.1 / particle.m;
            particle.u_vec.0 += particle.a_vec.0 * dt ;
            particle.u_vec.1 += particle.a_vec.1 * dt ;
            particle.p_vec.0 += particle.u_vec.0 * dt;
            particle.p_vec.1 += particle.u_vec.1 * dt;
        }
        // Draw bodies
        for particle in bodies.iter() {
            mq::draw_circle(
                particle.p_vec.0 as f32 * DRAW_SCALE as f32,
                particle.p_vec.1 as f32 * DRAW_SCALE as f32,
                2.0,
                mq::WHITE,
            );
        }
        mq::next_frame().await;
    };
}

// let mut iter = particles.iter_with_rest();
// while let Some((particle, other_particles)) = iter.next() {
//     let mut f_vec = (0.0, 0.0);
//     for other_particle in other_particles {
//         let dx = other_particle.p_vec.0 - particle.p_vec.0;
//         let dy = other_particle.p_vec.1 - particle.p_vec.1;
//         let r = (dx * dx + dy * dy).sqrt();
//         let f = G * particle.m * other_particle.m / (r * r);
//         let fx = f * dx / r;
//         let fy = f * dy / r;
//         f_vec.0 += fx;
//         f_vec.1 += fy;
//     }
//     particle.u_vec.0 += (f_vec.0 / particle.m) * dt ;
//     particle.u_vec.1 += (f_vec.1 / particle.m) * dt ;
//     particle.p_vec.0 += particle.u_vec.0 * dt;
//     particle.p_vec.1 += particle.u_vec.1 * dt;

// }