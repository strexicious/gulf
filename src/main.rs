use ggez;
use ggez::input::{
    mouse::MouseButton,
    keyboard::{
        KeyCode,
        KeyMods,
    }
};
use ggez::event;
use ggez::graphics::{self, DrawParam};
use ggez::nalgebra as na;

#[derive(Debug)]
struct BigMass {
    mass: f32,
    radius: f32,
}

impl BigMass {
    fn gravity(&self) -> f32 {
        6.674e-11 * self.mass * self.radius.powi(2)
    }
}

struct MainState {
    ball_pos: na::Point2<f32>,
    bodies: Vec<(na::Point2<f32>, BigMass)>,
    anchored: bool,
    mouse_pos: na::Point2<f32>,
    cur_vel: f32,
}

impl MainState {

    const BALL_MASS: f32 = 2.0;
    
    fn new() -> ggez::GameResult<MainState> {
        Ok(MainState {
            ball_pos: na::Point2::new(50.0, 50.0),
            bodies: vec![],
            anchored: false,
            mouse_pos: na::Point2::new(0.0, 0.0),
            cur_vel: 0.0
        })
    }

    fn get_forward(&self) -> na::Vector2<f32> {
        self.ball_pos - self.mouse_pos
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        const EPSILON: f32 = 1e-2;

        if self.cur_vel > EPSILON {
            let displ_vec = self.get_forward().normalize() * self.cur_vel;
            let displ_vec = self.bodies.iter().fold(displ_vec, |displ_vec, body| {
                let body_dir = body.0 - self.ball_pos;
                displ_vec + body_dir.normalize() * body.1.gravity()
            });
            self.ball_pos += displ_vec;
            self.cur_vel /= 2.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let ball_disc = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball_pos,
            10.0,
            2.0,
            [1.0, 1.0, 1.0, 1.0].into()
        )?;
        graphics::draw(ctx, &ball_disc, DrawParam::default())?;

        if self.anchored && self.ball_pos != self.mouse_pos {
            let arrow = graphics::Mesh::new_line(
                ctx, 
                &[self.mouse_pos, self.ball_pos + self.get_forward()], 
                2.0,
                [1.0, 1.0, 1.0, 1.0].into()
            )?;
            graphics::draw(ctx, &arrow, DrawParam::default())?;
        }

        for body in self.bodies.iter() {
            let body_disc = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                body.0,
                body.1.radius,
                2.0,
                [1.0, 0.5, 0.3, 1.0].into()
            )?;
            graphics::draw(ctx, &body_disc, DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        self.anchored = true;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _button: MouseButton,
        _x: f32,
        _y: f32
    ) {
        // F = ma, we take F = length of forward
        // we apply a = F / m instantaneouly to give velocity
        let force = self.get_forward().magnitude();
        self.cur_vel = force / Self::BALL_MASS;
        self.anchored = false;
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32
    ) {
        self.mouse_pos = na::Point2::new(x, y);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        match keycode {
            KeyCode::M => {
                self.bodies.push((
                    self.mouse_pos,
                    BigMass {
                        mass: 1e9,
                        radius: 10.0,
                    }
                ));
            },
            _ => ()
        }
    }
}

pub fn main() -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
