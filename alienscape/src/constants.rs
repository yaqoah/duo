// Characters (Alien, Clown and Ship) Properties
pub const INITIAL_ALIEN_X: f32 = 525.0;
pub const INITIAL_ALIEN_Y: f32 = 525.0;
pub const INITIAL_SPEED: f32 = 35.0;
pub const OPTIMUM_SPEED: f32 = 45.0;
pub const MAX_SPEED: f32 = 225.0;
pub const ACCELERATION: f32 = 0.1;
pub const BOOST: f32 = 0.5;
pub const OBJ_HEIGHT: f32 = 35.0;
pub const OBJ_WIDTH: f32 = 35.0;
pub static mut POWER_UP: bool = false;
pub static mut BOOSTABLE: bool = true;

// GGEZ Window Attributes
pub const WINDOW_HEIGHT: f32 = 595.0;
pub const WINDOW_WIDTH: f32 = 595.0;
pub const WINDOW_TITLE: &str = "ALIENSCAPE GAME V.0.1";
pub const GAME_ID: &str ="0.1";
pub const AUTHOR: &str ="Ahmed";

// Game Functionality Variables
pub const BOOST_TIME: u64 = 200;
pub const COOLDOWN_TIME: u64 = 1000;
pub const SCOPE_RADIUS: f32 = 130.0;
pub const ENEMIES: usize = 12; 

// Game Graphics and Screens
pub const TEXT_FONT_PATH: &str = r"\VIDEOPHREAK.ttf";
pub const SCOPE_PATH: &str = r"\scope.png";
pub const INTRO_PATH: &str = r"\intro.png";
pub const WIN_PATH: &str = r"\success.png";
pub const OVER_PATH: &str = r"\fail.png";
pub const ALIEN_PATH: &str = r"\alien.png";
pub const CLOWN_PATH: &str = r"\clown.png";
pub const SHIP_PATH: &str = r"\ship.png";