//! The universe, where everything to do with server-wide functions resides

pub mod world;

/// The maximum number of worlds to be hosted per universe
/// 
/// This is to be tested and will be increased, decreased, or set to a user-defined value prior to release
pub const MAX_WORLDS: u8 = 16;

/// The maximum number of players accepted by this universe
///
/// This is to be tested and will be set to a user-defined value (or perhaps automatically depending on hardware) prior to release
pub const MAX_PLAYERS: i32 = 200;