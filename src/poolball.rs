use na::{Vector2, Point2};
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
use graphics::Transformed;
use graphics::ellipse;
use num_traits::Zero;

#[derive(Clone, Debug, PartialEq)]
pub struct Poolball {
    pub position: Point2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub velocity: Vector2<f64>,
    pub ball_type: BallType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BallType {
    White,
    Red,
    Blue,
}

const MASS: f64 = 0.1;
const RADIUS: f64 = 0.01;

impl Poolball {
    // Creates a new Golfball with a initial position and velocity
    pub fn new(position: Point2<f64>, ball_type: BallType) -> Poolball {
        Poolball {
            position: position,
            radius: RADIUS,
            mass: MASS,
            velocity: Vector2::new(0.0, 0.0),
            ball_type: ball_type,
        }
    }

    // Updates the balls position using its current velocity, then updating velocity
    pub fn update(&mut self, delta_time: f64) {
        self.position += self.velocity * delta_time;
    }

    pub fn update_velocity(&mut self, acceleration: Vector2<f64>, delta_time: f64) {
        self.velocity += acceleration * delta_time;
    }

    // Sets the velocity of the ball to the given velocity
    pub fn set_velocity(&mut self, new_velocity: Vector2<f64>) {
        self.velocity = new_velocity.clone();
    }

    // Returns true if the poolball is stationary
    pub fn is_stationary(&self) -> bool {
        self.velocity.is_zero()
    }

    // Renders itself using the given graphics and ellipse
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.4];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let ellipse = match self.ball_type {
            BallType::White => ellipse::Ellipse::new(WHITE),
            BallType::Red => ellipse::Ellipse::new(RED),
            BallType::Blue => ellipse::Ellipse::new(BLUE),
        };

        gl.draw(args.viewport(), |c, gl| {

            let trans = c.transform
                .scale(args.width as f64, args.height as f64)
                .trans(self.position.x, self.position.y);
            // Draw the cue ball
            ellipse.draw(ellipse::circle(0.0, 0.0, self.radius),
                         &c.draw_state,
                         trans,
                         gl);
        });
    }
}

#[test]
fn test_update() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0), BallType::Red);
    ball.set_velocity(Vector2::new(1.0, 1.0));
    ball.update(1.0);
    assert_eq!(ball.position, Point2::new(1.0, 1.0));
}

#[test]
fn test_update_velocity() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0), BallType::Red);
    ball.update_velocity(Vector2::new(1.0, 1.0), 1.0);
    assert_eq!(ball.velocity, Vector2::new(1.0, 1.0));
}

#[test]
fn test_set_velocity() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0), BallType::Red);
    let new_velocity = Vector2::new(1.0, 1.0);
    ball.set_velocity(new_velocity);
    assert_eq!(ball.velocity, new_velocity);
}

#[test]
fn test_is_stationary() {
    let mut ball = Poolball::new(Point2::new(0.0, 0.0), BallType::Red);
    assert!(ball.is_stationary());
    ball.set_velocity(Vector2::new(1.0, 1.0));
    assert!(!ball.is_stationary());
}
