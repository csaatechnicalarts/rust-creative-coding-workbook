#![allow(warnings)]

use ::array_init::array_init;
use raylib::ffi::KeyboardKey::KEY_ENTER;
use raylib::prelude::*;
use std::process;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const PLAYER_LIFES: i32 = 5;
const BRICKS_LINES: i32 = 5;
const BRICKS_PER_LINE: i32 = 20;
const BRICKS_POSITION_Y: i32 = 50;

enum GameScreen {
    LOGO,
    TITLE,
    GAMEPLAY,
    ENDING,
}

struct Player {
    position: Vector2,
    speed: Vector2,
    size: Vector2,
    bounds: Rectangle,
    lifes: i32,
}

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    active: bool,
}

#[derive(Clone, Debug)]
struct Brick {
    position: Vector2,
    size: Vector2,
    bounds: Rectangle,
    resistance: i32,
    active: bool,
}

impl Brick {
    fn new() -> Brick {
        Brick {
            position: Vector2::new(0.0, 0.0),
            size: Vector2::new(0.0, 0.0),
            bounds: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            resistance: 0,
            active: false,
        }
    }
}

#[derive(Debug)]
struct BrickField {
    bricks: [Brick; (BRICKS_LINES * BRICKS_PER_LINE) as usize],
}

impl BrickField {
    fn new() -> BrickField {
        BrickField {
            bricks: array_init(|_| Brick::new()),
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Result<&mut Brick, &'static str> {
        if x >= BRICKS_LINES || y >= BRICKS_PER_LINE {
            return Err("Out bounds access to BrickField");
        }

        Ok(&mut self.bricks[((y * BRICKS_LINES) + x) as usize])
    }

    fn get(&mut self, x: i32, y: i32) -> Result<&Brick, &'static str> {
        let retval = self.get_mut(x, y).unwrap_or_else(|err| {
            println!("Game Logic: {} : x = {}, y = {}", err, x, y);
            process::exit(1);
        });

        // Upgrade the mutable reference to a more restrictive reference binding.
        return Ok(&*retval);
    }
}

fn main() {
    // ********************************************************
    // LESSON 01: Window initialization and screens management.
    // ********************************************************

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("03 PROJECT: BLOCKS GAME")
        .build();

    // Approx 1 frame refresh amounts to 1 second.
    rl.set_target_fps(60);

    let mut screenState = GameScreen::LOGO;
    let mut frames_counter: i64 = 0;
    let mut game_result = -1;
    let mut game_paused = false;

    let mut player = Player {
        position: Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT * 7 / 8) as f32),
        speed: Vector2::new(8.0, 0.0),
        size: Vector2::new(100.0, 24.0),
        bounds: Rectangle::new(0.0, 0.0, 0.0, 0.0),
        lifes: PLAYER_LIFES,
    };

    let mut ball = Ball {
        radius: 10.0,
        active: false,
        position: Vector2::new(
            player.position.x + player.size.x / 2.0,
            player.position.y - 20.0,
        ),
        speed: Vector2::new(4.0, 4.0),
    };

    let mut target_bricks = BrickField::new();

    for j in 0..BRICKS_LINES {
        for i in 0..BRICKS_PER_LINE {
            let brick = target_bricks.get_mut(j, i).unwrap_or_else(|err| {
                println!("Game Logic: {} : x = {}, y = {}", err, i, j);
                process::exit(1);
            });
            brick.size = Vector2::new((SCREEN_WIDTH / BRICKS_PER_LINE) as f32, 20.0);
            brick.position = Vector2::new(
                ((i as f32) * brick.size.x),
                (j as f32) * brick.size.y + (BRICKS_POSITION_Y as f32),
            );
            brick.bounds = Rectangle::new(
                brick.position.x,
                brick.position.y,
                brick.size.x,
                brick.size.y,
            );
            brick.resistance = 0;
            brick.active = true;
        }
    }

    //println!("\n {:?} \n", target_bricks);

    while !rl.window_should_close() {
        use raylib::consts::KeyboardKey::*;

        // ****************
        // Game state block
        // ****************

        match screenState {
            GameScreen::LOGO => {
                frames_counter += 1;
                if frames_counter > 180 {
                    // Change to TITLE screen after 3 seconds.
                    screenState = GameScreen::TITLE;
                    frames_counter = 0;
                }
            }
            GameScreen::TITLE => {
                frames_counter += 1;

                // ***********************************************
                // LESSON 03: Inputs management (keyboard, mouse).
                // ***********************************************

                if rl.is_key_pressed(KEY_ENTER) {
                    screenState = GameScreen::GAMEPLAY;
                }
            }
            GameScreen::GAMEPLAY => {
                if !game_paused {
                    if rl.is_key_pressed(KEY_P) {
                        game_paused = !game_paused;
                    }

                    if !game_paused {
                        // Player movement
                        if rl.is_key_down(KEY_LEFT) {
                            player.position.x -= player.speed.x;
                        }
                        if rl.is_key_down(KEY_RIGHT) {
                            player.position.x += player.speed.x;
                        }

                        if player.position.x <= 0.0 {
                            player.position.x = 0.0;
                        }
                        if (player.position.x + player.size.x) >= (SCREEN_WIDTH as f32) {
                            player.position.x = (SCREEN_WIDTH as f32) - player.size.x;
                        }

                        player.bounds = Rectangle::new(
                            player.position.x,
                            player.position.y,
                            player.size.x,
                            player.size.y,
                        );

                        if ball.active {
                            // Ball movement logic
                            ball.position.x += ball.speed.x;
                            ball.position.y += ball.speed.y;

                            // Collision logic: ball vs screen-limits
                            if (ball.position.x + ball.radius) > (SCREEN_WIDTH as f32)
                                || (ball.position.x - ball.radius) <= 0.0
                            {
                                ball.speed *= -1.0;
                            }
                            if (ball.position.y - ball.radius) <= 0.0 {
                                ball.speed *= -1.0;
                            }

                            // Game ending logic
                            if (ball.position.y + ball.radius) >= (SCREEN_HEIGHT as f32) {
                                ball.position.x = player.position.x + player.size.x / 2.0;
                                ball.position.y = player.position.y - ball.radius - 1.0;
                                ball.speed.x = 0.0;
                                ball.speed.y = 0.0;
                                ball.active = false;

                                player.lifes -= 1;
                            }

                            if player.lifes < 0 {
                                screenState = GameScreen::ENDING;
                                player.lifes = PLAYER_LIFES;
                                frames_counter = 0;
                            }
                        } else {
                            // Ret ball position to track player's position
                            ball.position.x = player.position.x + player.size.x / 2.0;

                            if rl.is_key_pressed(KEY_SPACE) {
                                // Activate the ball and resume the game
                                ball.active = true;
                                ball.speed = Vector2::new(0.0, -5.0);
                            }
                        }
                    }
                }
            }
            GameScreen::ENDING => {
                frames_counter += 1;
                if rl.is_key_pressed(KEY_ENTER) {
                    screenState = GameScreen::TITLE;
                }
            }
            _ => {
                println!("Warning: Unknown screen state.");
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // **********
        // Draw block
        // **********

        match screenState {
            GameScreen::LOGO => {
                d.draw_text("LOGO SCREEN", 20, 20, 40, Color::LIGHTGRAY);
                d.draw_text("WAIT for 3 SECONDS ...", 290, 220, 20, Color::GRAY);
            }
            GameScreen::TITLE => {
                //d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::GREEN);
                d.draw_text("TITLE SCREEN", 20, 20, 40, Color::DARKGREEN);
                if (frames_counter / 30) % 2 == 0 {
                    d.draw_text(
                        "PRESS [ENTER] to START",
                        d.get_screen_width() / 2 - d.measure_text("PRESS [ENTER] to START", 20) / 2,
                        d.get_screen_height() / 2 + 60,
                        20,
                        Color::DARKGREEN,
                    );
                }
            }
            GameScreen::GAMEPLAY => {
                // *************************************************
                // LESSON 02: Draw basic shapes (circle, rectangle).
                // *************************************************
                d.draw_rectangle(
                    player.position.x as i32,
                    player.position.y as i32,
                    player.size.x as i32,
                    player.size.y as i32,
                    Color::BLACK,
                );

                d.draw_circle_v(ball.position, ball.radius, Color::MAROON);

                for j in 0..BRICKS_LINES {
                    for i in 0..BRICKS_PER_LINE {
                        let brick = target_bricks.get(j, i).unwrap_or_else(|err| {
                            println!("Draw Logic: {} : x = {}, y = {}", err, i, j);
                            process::exit(1);
                        });
                        if brick.active {
                            if (i + j) % 2 == 0 {
                                d.draw_rectangle(
                                    (brick.position.x as i32),
                                    (brick.position.y as i32),
                                    (brick.size.x as i32),
                                    (brick.size.y as i32),
                                    Color::GRAY,
                                );
                            } else {
                                d.draw_rectangle(
                                    (brick.position.x as i32),
                                    (brick.position.y as i32),
                                    (brick.size.x as i32),
                                    (brick.size.y as i32),
                                    Color::DARKGRAY,
                                );
                            }
                        }
                    }
                }

                for i in 0..player.lifes {
                    d.draw_rectangle(20 + 40 * i, SCREEN_HEIGHT - 30, 35, 10, Color::LIGHTGRAY);
                }

                if game_paused {
                    d.draw_text(
                        "GAME PAUSED",
                        (SCREEN_WIDTH / 2) - d.measure_text("GAME PAUSED", 40) / 2,
                        (SCREEN_HEIGHT / 2) + 60,
                        40,
                        Color::GRAY,
                    );
                }
            }
            GameScreen::ENDING => {
                d.draw_text("ENDING SCREEN", 20, 20, 40, Color::DARKBLUE);

                if ((frames_counter / 30) % 2 == 0) {
                    d.draw_text(
                        "PRESS [ENTER] to PLAY AGAIN",
                        d.get_screen_width() / 2
                            - d.measure_text("PRESS [ENTER] to PLAY AGAIN", 20) / 2,
                        d.get_screen_height() / 2 + 80,
                        20,
                        Color::GRAY,
                    );
                }
            }
            _ => (),
        }

        /*
        for i in 0..player.lifes {
            d.draw_rectangle(20 + 40*i, SCREEN_HEIGHT - 30, 35, 10, Color::LIGHTGRAY);
        }
        */
    }
}
