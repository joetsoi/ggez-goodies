extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::mint::{Point2, Vector2};
use ggez::timer;
use ggez::{Context, GameResult};

extern crate ggez_goodies;
use ggez_goodies::particle::*;

struct MainState {
    particles: ParticleSystem,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let system = ParticleSystemBuilder::new(ctx)
            .count(10000)
            .emission_rate(2000.0)
            .acceleration(Vector2 { x: 0.0, y: 50.0 })
            .start_max_age(5.0)
            .start_size_range(2.0, 15.0)
            .start_color_range(
                graphics::Color::from((0, 0, 0)),
                graphics::Color::from((255, 255, 255)),
            )
            .start_velocity_range(
                Vector2 {
                    x: -50.0,
                    y: -200.0,
                },
                Vector2 { x: 50.0, y: 0.0 },
            )
            .start_ang_vel_range(-10.0, 10.0)
            .delta_size(Transition::range(15.0, 5.0))
            .delta_color(Transition::range(
                ggez::graphics::Color::from((255, 0, 0)),
                ggez::graphics::Color::from((255, 255, 0)),
            ))
            .emission_shape(EmissionShape::Circle(Point2 { x: 0.0, y: 0.0 }, 150.0))
            //.emission_shape(EmissionShape::Line(Point2::new(-100.0, -100.0), Point2::new(100.0, 100.0)))
            .build();
        let state = MainState { particles: system };
        Ok(state)
    }
}

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.particles.update(seconds);
            if timer::ticks(ctx) % 10 == 0 {
                println!(
                    "Particles: {}, FPS: {}",
                    self.particles.count(),
                    timer::fps(ctx)
                );
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        graphics::draw(
            ctx,
            &mut self.particles,
            (Point2 {
                x: WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            },),
        )?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("shiny_particles", "test")
        .window_setup(conf::WindowSetup::default().title("Shiny particles"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .unwrap();

    let game = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, event_loop, game) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
