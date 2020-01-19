use tetra::{Context, ContextBuilder, State, Event};
use tetra::graphics::{self, Color, Texture, Rectangle, DrawParams};
use tetra::input::{self, Key};
use tetra::graphics::animation::Animation;
use tetra::math::Vec2;
use std::time::Duration;
use std::f32;
use std::f32::consts::PI;

mod entities;
use entities::{Player, Entitie, Ship, Engine, Rotation, Bullet, EntitieType, Enemy};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const SCALE: i32 = 1;

struct Assets{
    scale: f32,
    view_width: f32,
    view_height: f32,
    spaceship1_on: Animation,
    spaceship1_left: Animation,
    spaceship1_right: Animation,
    spaceship1_off: Animation,
    spaceship2_on: Animation,
    bullet1: Animation,
}

impl Assets {
    fn new(ctx: &mut Context, w: i32, h: i32, s:i32) -> tetra::Result<Assets> {
        let texture_l = Texture::new(ctx, "./assets/spaceshipL.png")?;
        let texture_r = Texture::new(ctx, "./assets/spaceshipR.png")?;
        let texture_on = Texture::new(ctx,"./assets/spaceshipF.png")?;
        let texture_off = Texture::new(ctx,"./assets/spaceshipS.png")?;
        let texture_bullet1 = Texture::new(ctx, "./assets/bullet1.png")?;
        let texture_ship2_on = Texture::new(ctx, "./assets/spaceship2F.png")?;
        let anim_time = 0.05;
        Ok(Assets {
            scale: s as f32,
            view_width: w as f32,
            view_height: h as f32,
            spaceship1_left:  Animation::new(
                texture_l.clone(),
                Rectangle::row(0.0,0.0,46.0,45.0).take(5).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            spaceship1_right:  Animation::new(
                texture_r.clone(),
                Rectangle::row(0.0,0.0,46.0,45.0).take(5).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            spaceship1_on:  Animation::new(
                texture_on.clone(),
                Rectangle::row(0.0,0.0,46.0,45.0).take(5).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            spaceship1_off:  Animation::new(
                texture_off.clone(),
                Rectangle::row(0.0,0.0,46.0,45.0).take(1).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            bullet1:  Animation::new(
                texture_bullet1.clone(),
                Rectangle::row(0.0,0.0,8.0,17.0).take(1).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            spaceship2_on:  Animation::new(
                texture_ship2_on.clone(),
                Rectangle::row(0.0,0.0,38.0,40.0).take(5).collect(),
                Duration::from_secs_f64(anim_time),
            ),
            
        })
    }
    fn entitie_render<T: Entitie> (&mut self, ctx: &mut Context, ent: &T) {
        let mut ship_animation = &self.spaceship1_off;
        match ent.get_entitie_type() {
            EntitieType::Ship1 => {
                match ent.get_move_state() {
                    Ship::EngineOn => {
                        self.spaceship1_on.advance(ctx);
                        ship_animation = &self.spaceship1_on;
                        },
                    Ship::TurnLeft => {
                        self.spaceship1_left.advance(ctx);
                        ship_animation = &self.spaceship1_left;
                        },
                    Ship::TurnRight => {
                        self.spaceship1_right.advance(ctx);
                        ship_animation = &self.spaceship1_right;
                        },
                    Ship::EngineOff => {ship_animation = &self.spaceship1_off},
                    
                    _ => {},
                }
            },
            EntitieType::Bullet1 => {
                self.bullet1.advance(ctx);
                ship_animation = &self.bullet1;
            },
            EntitieType::Ship2 => {
                self.spaceship2_on.advance(ctx);
                ship_animation = &self.spaceship2_on;

            },
            _ => {}
        
        }
        graphics::draw(
            ctx,
            ship_animation,
            DrawParams::new()
                .position(Vec2::new(ent.get_x() * self.scale, ent.get_y() * self.scale))
                .origin(Vec2::new(0.5 * ent.get_width(), 0.5 * ent.get_height()))
                .rotation(ent.get_theta() + 0.5 * PI)
                .scale(Vec2::new(self.scale, self.scale)),
        )
    }
    // fn entitie_render(&mut self, ctx: &mut Context, ent: &dyn Entitie) {
    //     let mut ship_animation = &self.spaceship1_off;
    //     match ent.get_entitie_type() {
    //         EntitieType::Ship1 => {
    //             match ent.get_move_state() {
    //                 Ship::EngineOn => {
    //                     self.spaceship1_on.advance(ctx);
    //                     ship_animation = &self.spaceship1_on;
    //                     },
    //                 Ship::TurnLeft => {
    //                     self.spaceship1_left.advance(ctx);
    //                     ship_animation = &self.spaceship1_left;
    //                     },
    //                 Ship::TurnRight => {
    //                     self.spaceship1_right.advance(ctx);
    //                     ship_animation = &self.spaceship1_right;
    //                     },
    //                 Ship::EngineOff => {ship_animation = &self.spaceship1_off},
                    
    //                 _ => {},
    //             }
    //         },
    //         EntitieType::Bullet1 => {
    //             self.bullet1.advance(ctx);
    //             ship_animation = &self.bullet1;
    //         },
    //         EntitieType::Ship2 => {
    //             self.spaceship2_on.advance(ctx);
    //             ship_animation = &self.spaceship2_on;

    //         },
    //         _ => {}
        
    //     }
    //     graphics::draw(
    //         ctx,
    //         ship_animation,
    //         DrawParams::new()
    //             .position(Vec2::new(ent.get_x() * self.scale, ent.get_y() * self.scale))
    //             .origin(Vec2::new(0.5 * ent.get_width(), 0.5 * ent.get_height()))
    //             .rotation(ent.get_theta() + 0.5 * PI)
    //             .scale(Vec2::new(self.scale, self.scale)),
    //     )
    // }
}



struct GameState {
    assets: Assets,
    player: Player,
    bullet_list: Vec<Bullet>,
    enemies_list: Vec<Enemy>,
    position_list: Vec<(f32, f32)>,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut enemies_list = Vec::<Enemy>::new();
        enemies_list.push(Enemy::new(300.0,300.0, PI * 0.25)?);
        Ok(GameState{
            assets: Assets::new(ctx, WIDTH, HEIGHT,SCALE)?,
            player: Player::new()?,
            bullet_list: Vec::<Bullet>::new(),
            enemies_list,
            position_list: Vec::new(),
        })
    }

    fn fire(&mut self,x: f32, y: f32, t: f32) {
        self.bullet_list.push(Bullet::new(x, y, t));
    }

    fn keyboard(&mut self, ctx: &mut Context) {
        if input::is_key_down(ctx, Key::Up) {
            self.player.go(Engine::EngineOn, Rotation::Skip);
            // self.player.engine_on(true);
        } else {
            self.player.go(Engine::EngineOff, Rotation::Skip);
            // self.player.engine_on(false);
        }

        if input::is_key_down(ctx, Key::Space) {
            if self.player.fire() {
                let theta = self.player.get_theta();
                self.fire(self.player.get_x(), self.player.get_y(), theta);
            };
        } else {
            self.player.allow_fire();
        }

        if input::is_key_down(ctx, Key::Left) {
            self.player.go(Engine::Skip, Rotation::TurnLeft);
            // self.player.turn_left(true);
        } else if input::is_key_down(ctx, Key::Right) {
            self.player.go(Engine::Skip, Rotation::TurnRight);
            // self.player.turn_right(true);
        } else {
            self.player.go(Engine::Skip, Rotation::None);
            // self.player.turn_left(false);
            // self.player.turn_right(false);
        }
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.keyboard(ctx);
        for bullet in self.bullet_list.iter_mut() {
            bullet.update();
        }
        self.position_list.clear();
        for enemy in self.enemies_list.iter() {
            self.position_list.push((enemy.get_x(), enemy.get_y()));
        }
        for enemy in self.enemies_list.iter_mut() {
            enemy.update(&self.player, &self.position_list)
        }
        self.player.update();
        graphics::clear(ctx,Color::rgb(0.039, 0.058, 0.092));
        for enemy in self.enemies_list.iter() {
            self.assets.entitie_render(ctx, enemy);
        }
        for bullet in self.bullet_list.iter() {
            self.assets.entitie_render(ctx, bullet);
        }
        self.assets.entitie_render(ctx, &self.player);
        Ok(())
    }
}
fn main() -> tetra::Result {
    ContextBuilder::new("Rusty Spaceship", SCALE * WIDTH, SCALE * HEIGHT)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}
