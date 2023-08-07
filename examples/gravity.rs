#![allow(unused_imports, dead_code)]
use craig_iter::{IntoNextWithRestIterator, LendingIterator};
use rand::Rng;
use macroquad::prelude as mq;

const G: f64 = 6.674e-11;
const SOLAR_MASS: f64 = 1.989e30; 
const EARTH_MASS: f64 = 5.972e24;
const AU: f64 = 1.496e11;
const EARTH_ORBIT_SPEED: f64 = 2.978e4;
const UNIVERSE_SIZE: f64 = 10.0*AU;
const SCREEN_SIZE : f64 = 600.0; 
const DRAW_SCALE: f64 = SCREEN_SIZE / UNIVERSE_SIZE;
const SECS_IN_YEAR: f64 = 3.154e+7;
const TIME_SCALE: f64 = SECS_IN_YEAR / 10.0;

struct Particle {
    p_vec: (f64, f64),
    u_vec: (f64, f64),
    a_vec: (f64, f64),
    f_vec: (f64, f64),
    m: f64,
}

impl Particle {
    #[allow(dead_code)]
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


    //create sun in center of universe and earth in orbit
    let mut sun = Particle::new(
        (UNIVERSE_SIZE / 2.0, UNIVERSE_SIZE / 2.0),
        (0.0, 0.0),
        (0.0, 0.0),
        (0.0, 0.0),
        SOLAR_MASS,
    );
    //draw earth with 1 AU radius away from sun and earth orbit speed
    let mut earth = Particle::new(
        (UNIVERSE_SIZE / 2.0 + AU, UNIVERSE_SIZE / 2.0),
        (0.0, EARTH_ORBIT_SPEED),
        (0.0, 0.0),
        (0.0, 0.0),
        EARTH_MASS,
    );

    loop {
        mq::clear_background(mq::BLACK);
        // //Update particles
        let dt = mq::get_frame_time() as f64 * TIME_SCALE;
        //update forces
        let dx = earth.p_vec.0 - sun.p_vec.0;
        let dy = earth.p_vec.1 - sun.p_vec.1;
        let r = (dx * dx + dy * dy).sqrt();
        let f = -G * sun.m * earth.m / (r * r);
        let fx = f * dx / r;
        let fy = f * dy / r;
        earth.f_vec.0 = fx;
        earth.f_vec.1 = fy;
        sun.f_vec.0 = -fx;
        sun.f_vec.1 = -fy;
        // //update acceleration
        earth.a_vec.0 = earth.f_vec.0 / earth.m;
        earth.a_vec.1 = earth.f_vec.1 / earth.m;
        sun.a_vec.0 = sun.f_vec.0 / sun.m;
        sun.a_vec.1 = sun.f_vec.1 / sun.m;
        //update velocity
        earth.u_vec.0 += earth.a_vec.0 * dt;
        earth.u_vec.1 += earth.a_vec.1 * dt;
        sun.u_vec.0 += sun.a_vec.0 * dt;
        sun.u_vec.1 += sun.a_vec.1 * dt;
        // //update position
        earth.p_vec.0 += earth.u_vec.0 * dt;
        earth.p_vec.1 += earth.u_vec.1 * dt;
        sun.p_vec.0 += sun.u_vec.0 * dt;
        sun.p_vec.1 += sun.u_vec.1 * dt;

        // Draw bodies
        mq::draw_circle(
            (sun.p_vec.0 * DRAW_SCALE) as f32,
            (sun.p_vec.1 * DRAW_SCALE) as f32,
            10.0,
            mq::YELLOW,
        );
        mq::draw_circle(
            (earth.p_vec.0 * DRAW_SCALE) as f32,
            (earth.p_vec.1 * DRAW_SCALE) as f32,
            5.0,
            mq::BLUE,
        );
        //draw fvec
        mq::draw_line(
            (earth.p_vec.0 * DRAW_SCALE) as f32,
            (earth.p_vec.1 * DRAW_SCALE) as f32,
            (earth.p_vec.0 * DRAW_SCALE + earth.f_vec.0 * DRAW_SCALE/100000000000.0) as f32,
            (earth.p_vec.1 * DRAW_SCALE + earth.f_vec.1 * DRAW_SCALE/100000000000.0) as f32,
            1.0,
            mq::WHITE,
        );
        //write a_vec and u_vec as text
        mq::draw_text(
            &format!("a_vec: ({:.10}, {:.10})", earth.a_vec.0, earth.a_vec.1),
            10.0,
            20.0,
            20.0,
            mq::WHITE,
        );
        mq::draw_text(
            &format!("u_vec: ({:.10}, {:.10})", earth.u_vec.0, earth.u_vec.1),
            10.0,
            40.0,
            20.0,
            mq::WHITE,
        );
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