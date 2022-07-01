
static LOWEST_DIFFICULTY_COMPACT: u32            = 4294967294; // Preset difficulty value before first adjustment of difficulty
static USED_VERSION_V2_ABOVE_BLOCK_HEIGHT: u64   = 288 * 160;  // Start using the new algorithm after the 160th difficulty cycle
static USED_V2BLOCK_TIME_ABOVE_BLOCK_HEIGHT: u64 = 288 * 450;  // In the past, the code actually only queried the total time of the first 287 blocks. After a certain height is added 300 seconds, it becomes 288 blocks


