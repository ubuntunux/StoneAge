// editor
pub const EDITOR_CAMERA_MOVE_SPEED: f32 = 20.0;
pub const EDITOR_CAMERA_PAN_SPEED: f32 = 0.05;
pub const EDITOR_CAMERA_ROTATION_SPEED: f32 = 0.005;

// game constant
pub const UPPER_CAMERA_OFFSET_Y: f32 = 0.0;
pub const BOTTOM_CAMERA_OFFSET_Y: f32 = 1.0;
pub const CAMERA_POSITION_Y_MIN: f32 = 14.0;
pub const CAMERA_OFFSET_Y: f32 = 1.5;
pub const CAMERA_PITCH: f32 = 0.0;
pub const CAMERA_DISTANCE_MIN: f32 = 4.0;
pub const CAMERA_DISTANCE_MAX: f32 = 10.0;
pub const CAMERA_ZOOM_SPEED: f32 = 4.0;

// player
pub const GRAVITY: f32 = 30.0;
pub const GROUND_HEIGHT: f32 = 9.0;
pub const ATTACK_TIME: f32 = 0.15;
pub const MOVE_LIMIT: f32 = 2.0;
pub const BLOCK_TOLERANCE: f32 = 0.5;
pub const EAT_FOOD_DISTANCE: f32 = 1.0;

// npc
pub const NPC_IDLE_TERM_MIN: f32 = 3.0;
pub const NPC_IDLE_TERM_MAX: f32 = 10.0;
pub const NPC_IDLE_PLAY_MIN: f32 = 1.0;
pub const NPC_IDLE_PLAY_MAX: f32 = 2.0;
pub const NPC_ATTACK_TERM_MIN: f32 = 1.0;
pub const NPC_ATTACK_TERM_MAX: f32 = 2.0;
pub const NPC_ATTACK_DISTANCE: f32 = 1.0;
pub const NPC_TRACKING_RANGE_X: f32 = 5.0;
pub const NPC_TRACKING_RANGE_Y: f32 = 1.0;
pub const NPC_ROAMING_TERM: f32 = 5.0;
pub const NPC_AVAILABLE_MOVING_ATTACK: bool = true;

// AUDIO DATA
pub const AUDIO_ATTACK: &str = "swoosh";
pub const AUDIO_DEAD: &str = "pain_short";
pub const AUDIO_HIT: &str = "hit";
pub const AUDIO_CRUNCH: &str = "crunch";
pub const GAME_MUSIC: &str = "game_music";

// EFFECT DATA
pub const EFFECT_HIT: &str = "hit_effect";