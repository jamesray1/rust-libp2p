// Overlay parameters

/// The target number of peers in the mesh to gossip to and from.
pub const TARGET_MESH_DEGREE: u32 = 6;
/// Low water mark for the mesh degree, any lower and it could take longer to
/// find messages.
pub const LOW_WM_MESH_DEGREE: u32 = 4;
/// High water mark for the mesh degree, any higher and it could be too
/// much for bandwidth (particularly for low-end devices).
pub const HIGH_WM_MESH_DEGREE: u32 = 12;

// Gossip parameters
/// length of gossip history
pub const GOSSIP_HIST_LEN: u32 = 5;
/// This is the last index in the `MCache's` history window. We get
/// message IDs from up to this index.
pub const HISTORY_GOSSIP: u32 = 3;

/// length of total message history
pub const MSG_HIST_LEN: u32 = 120;
pub const SEEN_MSGS_CACHE: u32 = 120;

// hearbeat interval
pub const HEARTBEAT_INITIAL_DELAY: u32 = 100; // milliseconds
pub const HEARTBEAT_INTERVAL: u32 = 1;   // seconds.

pub const FANOUT_TTL: u32 = 60; // seconds
