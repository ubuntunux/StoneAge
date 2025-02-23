// editor
pub const EDITOR_CAMERA_MOVE_SPEED: f32 = 20.0;
pub const EDITOR_CAMERA_PAN_SPEED: f32 = 0.05;
pub const EDITOR_CAMERA_ROTATION_SPEED: f32 = 0.005;

// game constant
pub const CAMERA_PITCH_MIN: f32 = 15.0;
pub const CAMERA_PITCH_MAX: f32 = 30.0;
pub const CAMERA_DISTANCE_MIN: f32 = 5.0;
pub const CAMERA_DISTANCE_MAX: f32 = 10.0;
pub const CAMERA_OFFSET_Y: f32 = 1.5;
pub const CAMERA_ZOOM_SPEED: f32 = 4.0;
pub const CAMERA_ROTATION_SPEED_MIN: f32 = 0.5;
pub const CAMERA_ROTATION_SPEED_MAX: f32 = 20.0;

// player
pub const GRAVITY: f32 = 30.0;
pub const GROUND_HEIGHT: f32 = 0.0;
pub const MOVE_LIMIT: f32 = 2.0;
pub const BLOCK_TOLERANCE: f32 = 0.5;
pub const EAT_ITEM_DISTANCE: f32 = 1.0;
pub const CHARACTER_ROTATION_SPEED: f32 = 20.0;

// stamina
pub const MAX_STAMINA: f32 = 100.0;
pub const STAMINA_RECOVERY: f32 = 40.0;
pub const STAMINA_ATTACK: f32 = 10.0;
pub const STAMINA_POWER_ATTACK: f32 = 30.0;
pub const STAMINA_RUN: f32 = 30.0;
pub const STAMINA_JUMP: f32 = 15.0;
pub const STAMINA_ROLL: f32 = 30.0;

// npc
pub const NPC_IDLE_TERM_MIN: f32 = 1.0;
pub const NPC_IDLE_TERM_MAX: f32 = 3.0;
pub const NPC_ATTACK_TERM_MIN: f32 = 1.0;
pub const NPC_ATTACK_TERM_MAX: f32 = 2.0;
pub const NPC_ATTACK_RANGE: f32 = 0.1;
pub const NPC_ATTACK_HIT_RANGE: f32 = 0.5;
pub const NPC_TRACKING_RANGE: f32 = 5.0;
pub const NPC_ROAMING_RADIUS: f32 = 5.0;
pub const NPC_ROAMING_TIME: f32 = 5.0;
pub const NPC_AVAILABLE_MOVING_ATTACK: bool = false;

// AUDIO DATA
pub const AUDIO_ATTACK: &str = "swoosh";
pub const AUDIO_HIT: &str = "hit";
pub const AUDIO_CRUNCH: &str = "crunch";
pub const AUDIO_FOOTSTEP: &str = "footstep";
pub const AUDIO_ROLL: &str = "roll";
pub const AUDIO_JUMP: &str = "jump";
pub const AMBIENT_SOUND: &str = "ambient_sound";
pub const GAME_MUSIC: &str = "game_music";

// UI
pub const MATERIAL_INTRO_IMAGE: &str = "ui/intro_image";
pub const MATERIAL_CROSS_HAIR: &str = "ui/cross_hair_box";

// EFFECT DATA
pub const EFFECT_HIT: &str = "hit_effect";

// Items
pub const ITEM_MEAT: &str = "items/meat";

// Game Scene
pub const GAME_SCENE_INTRO: &str = "game_scenes/intro_stage";