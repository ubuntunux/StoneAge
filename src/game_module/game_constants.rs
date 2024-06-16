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
pub const PLAYER_WALK_SPEED: f32 = 3.0;
pub const PLAYER_RUN_SPEED: f32 = PLAYER_WALK_SPEED * 1.8;
pub const PLAYER_ROLL_SPEED: f32 = PLAYER_WALK_SPEED * 1.5;
pub const PLAYER_JUMP_SPEED: f32 = 13.0;
pub const GRAVITY: f32 = 30.0;
pub const GROUND_HEIGHT: f32 = 9.0;
pub const CONTINUOUS_ATTACK_TIME: f32 = 0.15;
pub const ATTACK_TIME: f32 = 0.15;
pub const MOVE_LIMIT: f32 = 2.0;
pub const BLOCK_TOLERANCE: f32 = 0.5;
pub const EAT_FOOD_DISTANCE: f32 = 1.0;

// npc
pub const NPC_ATTACK_TERM: f32 = 2.0;
pub const NPC_ATTACK_DISTANCE: f32 = 1.0;
pub const NPC_ROAMING_TERM: f32 = 10.0;

// AUDIO DATA
pub const AUDIO_ATTACK: &str = "swoosh";
pub const AUDIO_DEAD: &str = "pain_short";
pub const AUDIO_HIT: &str = "hit";
pub const AUDIO_CRUNCH: &str = "crunch";

// EFFECT DATA
pub const EFFECT_HIT: &str = "hit_effect";