use std::f32;
use std::f32::consts::PI;

pub trait Entitie {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn get_theta(&self) -> f32;
    fn get_move_state(&self) -> &Ship;
    fn get_entitie_type(&self) -> &EntitieType;
    // fn go(&mut self, _: Engine, _: Rotation);


}

pub enum EntitieType {
    Ship1,
    Ship2,
    Bullet1,

}

pub enum Ship {
    EngineOff,
    EngineOn,
    TurnLeft,
    TurnRight,
    BulletOn,
    None,
}

pub enum Engine {
    EngineOn,
    EngineOff,
    Skip
}

pub enum Rotation {
    TurnLeft,
    TurnRight,
    Skip,
    None,
}

pub struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    theta: f32,
    vx: f32,
    vy: f32,
    max_speed: f32,
    engine_on: bool,
    turn_left: bool,
    turn_right: bool,
    fire_allowed: bool,
    ship_move: Ship,
    ship_type: EntitieType,
}

impl Player {
    pub fn new() -> tetra::Result<Player> {
        Ok(Player {
            x: 200.0,
            y: 200.0,
            w: 46.0,
            h: 45.0,
            vx: 0.0,
            vy: 0.0,
            max_speed: 8.0,
            theta:  0.0,
            ship_move: Ship::EngineOff,
            ship_type: EntitieType::Ship1,
            engine_on: false,
            turn_left: false,
            turn_right: false,
            fire_allowed: true,
        })
    }

    pub fn update(&mut self) {
        if self.turn_left {
            self.theta -=0.07;
        } else if self.turn_right {
            self.theta += 0.07;
        }
        let squared_speed = self.vx.powi(2) + self.vy.powi(2);
        if self.engine_on && squared_speed < self.max_speed.powi(2) {
            self.vx += 0.1 * self.theta.cos();
            self.vy += 0.1 * self.theta.sin();
        } else {
            self.vx *= 0.98;
            self.vy *= 0.98;
        }

        self.x += self.vx;
        self.y += self.vy;
        
    }
    pub fn fire(&mut self) -> bool{
        if self.fire_allowed {
            self.fire_allowed = false;
            true
        } else {
            false
        }
    }
    pub fn allow_fire(&mut self) {
        self.fire_allowed = true;
    }
    pub fn go(&mut self,engine: Engine, turn: Rotation) {
        match engine {
            Engine::EngineOn => self.engine_on = true,
            Engine::EngineOff => self.engine_on = false,
            _ =>{}
        }

        match turn {
            Rotation::TurnLeft => {self.turn_left = true; self.turn_right = false;}
            Rotation::TurnRight => {self.turn_left = false; self.turn_right = true;},
            Rotation::None => {self.turn_left = false; self.turn_right = false;},

            _ =>{}
        }

        if self.turn_left {
            self.ship_move = Ship::TurnLeft;
        } else if self.turn_right {
            self.ship_move = Ship::TurnRight;
        } else if !self.engine_on {
            self.ship_move = Ship::EngineOff;
        }
        if self.engine_on {
            self.ship_move = Ship::EngineOn;
        }
    }

}

impl Entitie for Player {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_width(&self) -> f32 {
        self.w
    }
    fn get_height(&self) -> f32 {
        self.h
    }
    fn get_theta(&self) -> f32 {
        self.theta
    }
    fn get_move_state(&self) -> &Ship {
        &self.ship_move
    }
    fn get_entitie_type(&self) -> &EntitieType {
        &self.ship_type
    }

    
    
}

pub struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    theta: f32,
    w: f32,
    h: f32,
    bullet_move: Ship,
    bullet_type: EntitieType,
}

impl Bullet {
    pub fn new(xo: f32, yo: f32, to: f32) -> Bullet {
        let theta = to;
        Bullet{
            x: xo,
            y: yo,
            theta,
            vx: 10.0 * theta.cos(),
            vy: 10.0 * theta.sin(),
            w: 8.0, 
            h: 17.0,
            bullet_move: Ship::None,
            bullet_type: EntitieType::Bullet1,
        }
    } 

    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

impl Entitie for Bullet {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_width(&self) -> f32 {
        self.w
    }
    fn get_height(&self) -> f32 {
        self.h
    }
    fn get_theta(&self) -> f32 {
        self.theta
    }
    fn get_move_state(&self) -> &Ship {
        &self.bullet_move
    }
    fn get_entitie_type(&self) -> &EntitieType {
        &self.bullet_type
    }
}

pub struct Enemy {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    theta: f32,
    w: f32,
    h: f32,
    engine_on: bool,
    turn_left: bool,
    turn_right: bool,
    enemy_move: Ship,
    enemy_type: EntitieType,
    view: Vec<(f32,f32)>,
}

impl Enemy {
    pub fn new(xo: f32, yo: f32, to: f32) -> tetra::Result<Enemy> {
        let theta = to;
        Ok(Enemy{
            x: xo,
            y: yo,
            theta,
            vx: 0.0,
            vy: 0.0,
            w: 38.0, 
            h: 40.0,
            engine_on: false,
            turn_left: false,
            turn_right: false,
            enemy_move: Ship::None,
            enemy_type: EntitieType::Ship2,
            view: Vec::new(),
        })
    }
    pub fn update(&mut self, player: &Player, postion_list: &Vec<(f32, f32)>) {
        let d = ((self.x - player.get_x()).powi(2) + (self.y - player.get_y()).powi(2)).sqrt();
        let dir = (player.get_y()- self.y).atan2( player.get_x() - self.x);
        if self.theta > dir {
            self.theta -= 0.07;
        } else if self.theta < dir {
            self.theta += 0.07;
        }
        println!("{} *** {}", 180.0 * dir / PI, 180.0 * self.theta / PI );

    }
}

impl Entitie for Enemy {
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
    fn get_width(&self) -> f32 {
        self.w
    }
    fn get_height(&self) -> f32 {
        self.h
    }
    fn get_theta(&self) -> f32 {
        self.theta
    }
    fn get_move_state(&self) -> &Ship {
        &self.enemy_move
    }
    fn get_entitie_type(&self) -> &EntitieType {
        &self.enemy_type
    }
}
