use ggez;
use ggez::{ Context, GameResult };
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard::{ self, KeyCode };

use ggez::nalgebra as alg;

use rand::{ self, thread_rng, Rng };

const MIDDLE_LINE_W: f32 = 1.0;

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 25.0;

const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

const PADDING: f32 = 40.0;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

const PLAYER_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 300.0;

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("PONG_LEPTO", "Lepto");
    let (context, event_loop) = &mut context_builder.build()?;

    graphics::set_window_title(context, "Pong");

    let mut state = MainState::new(context);
    event::run(context, event_loop, &mut state)?;

    Ok(())
}

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut alg::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let screen_h = graphics::drawable_size(ctx).1;
    let delta_time = ggez::timer::delta(ctx).as_secs_f32();

    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir * PLAYER_SPEED * delta_time;
    }

    clamp(&mut pos.y, RACKET_HEIGHT_HALF, screen_h - RACKET_HEIGHT_HALF);
}

fn randomize_vec(vec: &mut alg::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };

}

struct MainState {
    player_1_pos: alg::Point2<f32>,
    player_2_pos: alg::Point2<f32>,

    player_1_score: i32,
    player_2_score: i32,
    
    ball_pos: alg::Point2<f32>,
    ball_velocity: alg::Vector2<f32>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screem_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        
        let mut ball_velocity = alg::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos : alg::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            player_2_pos : alg::Point2::new(screen_w - RACKET_WIDTH_HALF - PADDING, screen_h_half),
            
            player_1_score : 0,
            player_2_score : 0,

            ball_pos : alg::Point2::new(screem_w_half, screen_h_half),
            ball_velocity : ball_velocity,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);

        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);

        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

        self.ball_pos += self.ball_velocity * delta_time;

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;

            randomize_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);

            self.player_2_score += 1;
        } else if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            
            randomize_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            
            self.player_1_score += 1;
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        let intersects_player_1 =
            self.ball_pos.x - BALL_SIZE_HALF < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        }
        
        let intersects_player_2 =
            self.ball_pos.x - BALL_SIZE_HALF < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_2 {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }

        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, _screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        
        graphics::clear(ctx, graphics::BLACK);
        
        // Instantiate Meshes
        let racket_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), racket_rect, graphics::WHITE)?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), ball_rect, graphics::WHITE)?;

        let middle_line_rect = graphics::Rect::new(-MIDDLE_LINE_W * 0.5, 0.0, MIDDLE_LINE_W, screen_h);
        let middle_line_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), middle_line_rect, graphics::WHITE)?;

        let mut draw_param = graphics::DrawParam::default();
        
        // Draw Game
        draw_param.dest = [screen_w_half, 0.0].into();
        graphics::draw(ctx, &middle_line_mesh, draw_param)?;

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;
        
        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;
        
        // Draw Score
        let score_text = graphics::Text::new(format!("{}            {}", self.player_1_score, self.player_2_score));
        
        let mut score_pos = alg::Point2::new(screen_w_half, 40.0);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= alg::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);

        draw_param.dest = score_pos.into();
        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;

        Ok(())
    }

}
