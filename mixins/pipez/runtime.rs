// Pipez Native Mod - pipe network logic for items, fluids, energy, and gases
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game.

/// The mod's id, name and version.
pub const ID: &str = "pipez";
pub const NAME: &str = "Pipez";
pub const VERSION: &str = "1.2.33";
pub const DESCRIPTION: &str = "Simple and efficient pipes for items, fluids, energy, and gases";

/// Pipe types matching the content mod
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum PipeType {
    Item = 0,
    Fluid = 1,
    Energy = 2,
    Gas = 3,
    Universal = 4,
}

impl PipeType {
    pub fn from_block_id(block_id: &str) -> Option<Self> {
        match block_id {
            "pipez:item_pipe" => Some(PipeType::Item),
            "pipez:fluid_pipe" => Some(PipeType::Fluid),
            "pipez:energy_pipe" => Some(PipeType::Energy),
            "pipez:gas_pipe" => Some(PipeType::Gas),
            "pipez:universal_pipe" => Some(PipeType::Universal),
            _ => None,
        }
    }

    pub fn base_transfer_rate(&self) -> u32 {
        match self {
            PipeType::Item => 64,
            PipeType::Fluid => 1000,
            PipeType::Energy => 256,
            PipeType::Gas => 500,
            PipeType::Universal => 128,
        }
    }

    pub fn can_connect_to(&self, other: PipeType) -> bool {
        matches!(self, PipeType::Universal)
            || matches!(other, PipeType::Universal)
            || *self == other
    }

    pub fn name(&self) -> &'static str {
        match self {
            PipeType::Item => "item",
            PipeType::Fluid => "fluid",
            PipeType::Energy => "energy",
            PipeType::Gas => "gas",
            PipeType::Universal => "universal",
        }
    }
}

/// Upgrade tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum UpgradeTier {
    Basic = 0,
    Improved = 1,
    Advanced = 2,
    Ultimate = 3,
    Infinity = 4,
}

impl UpgradeTier {
    pub fn from_item_id(item_id: &str) -> Option<Self> {
        match item_id {
            "pipez:basic_upgrade" => Some(UpgradeTier::Basic),
            "pipez:improved_upgrade" => Some(UpgradeTier::Improved),
            "pipez:advanced_upgrade" => Some(UpgradeTier::Advanced),
            "pipez:ultimate_upgrade" => Some(UpgradeTier::Ultimate),
            "pipez:infinity_upgrade" => Some(UpgradeTier::Infinity),
            _ => None,
        }
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(UpgradeTier::Basic),
            1 => Some(UpgradeTier::Improved),
            2 => Some(UpgradeTier::Advanced),
            3 => Some(UpgradeTier::Ultimate),
            4 => Some(UpgradeTier::Infinity),
            _ => None,
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            UpgradeTier::Basic => 1.5,
            UpgradeTier::Improved => 2.5,
            UpgradeTier::Advanced => 4.0,
            UpgradeTier::Ultimate => 8.0,
            UpgradeTier::Infinity => 16.0,
        }
    }

    pub fn max_upgrades(&self) -> u8 {
        match self {
            UpgradeTier::Basic => 1,
            UpgradeTier::Improved => 2,
            UpgradeTier::Advanced => 3,
            UpgradeTier::Ultimate => 4,
            UpgradeTier::Infinity => 5,
        }
    }
}

/// Direction enum for pipe connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum Direction {
    Down = 0,
    Up = 1,
    North = 2,
    South = 3,
    West = 4,
    East = 5,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Direction::Down,
        Direction::Up,
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    pub fn offset(&self) -> (i32, i32, i32) {
        match self {
            Direction::Down => (0, -1, 0),
            Direction::Up => (0, 1, 0),
            Direction::North => (0, 0, -1),
            Direction::South => (0, 0, 1),
            Direction::West => (-1, 0, 0),
            Direction::East => (1, 0, 0),
        }
    }
}

/// Position in 3D space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn offset(&self, dir: Direction) -> Self {
        let (dx, dy, dz) = dir.offset();
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }
}

/// Pipe network data stored in block entities
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipeNetworkData {
    pub network_id: u64,
    pub pipe_type: PipeType,
    pub connections: u8,   // bitmask of Direction
    pub upgrades: [u8; 5], // count per UpgradeTier
    pub transfer_rate: u32,
    pub filter: Option<ItemFilter>,
    pub redstone_mode: RedstoneMode,
    pub distribution_mode: DistributionMode,
}

impl Default for PipeNetworkData {
    fn default() -> Self {
        Self {
            network_id: 0,
            pipe_type: PipeType::Item,
            connections: 0,
            upgrades: [0; 5],
            transfer_rate: 0,
            filter: None,
            redstone_mode: RedstoneMode::Disabled,
            distribution_mode: DistributionMode::RoundRobin,
        }
    }
}

impl PipeNetworkData {
    pub fn recalculate_transfer_rate(&mut self) {
        let base = self.pipe_type.base_transfer_rate() as f32;
        let mut multiplier = 1.0;
        for (i, &count) in self.upgrades.iter().enumerate() {
            if count > 0 {
                if let Some(tier) = UpgradeTier::from_u8(i as u8) {
                    multiplier += tier.multiplier() * count as f32;
                }
            }
        }
        self.transfer_rate = (base * multiplier) as u32;
    }

    pub fn has_connection(&self, dir: Direction) -> bool {
        self.connections & (1 << dir as u8) != 0
    }

    pub fn set_connection(&mut self, dir: Direction, connected: bool) {
        if connected {
            self.connections |= 1 << dir as u8;
        } else {
            self.connections &= !(1 << dir as u8);
        }
    }
}

/// Item filter for pipes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemFilter {
    pub items: Vec<String>, // item IDs
    pub mode: FilterMode,
    pub nbt_sensitive: bool,
}

impl Default for ItemFilter {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            mode: FilterMode::Whitelist,
            nbt_sensitive: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum FilterMode {
    Whitelist = 0,
    Blacklist = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum RedstoneMode {
    Disabled = 0,
    ActiveOnSignal = 1,
    ActiveWithoutSignal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum DistributionMode {
    RoundRobin = 0,
    Balanced = 1,
    Priority = 2,
}

/// Pipe network manager
pub struct PipeNetworkManager {
    networks: std::collections::HashMap<u64, PipeNetwork>,
    block_to_network: std::collections::HashMap<BlockPos, u64>,
    next_network_id: std::sync::atomic::AtomicU64,
}

struct PipeNetwork {
    id: u64,
    pipe_type: PipeType,
    members: std::collections::HashSet<BlockPos>,
    total_transfer_rate: u32,
}

impl Default for PipeNetworkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PipeNetworkManager {
    pub fn new() -> Self {
        Self {
            networks: std::collections::HashMap::new(),
            block_to_network: std::collections::HashMap::new(),
            next_network_id: std::sync::atomic::AtomicU64::new(1),
        }
    }

    pub fn get_or_create_network(&mut self, pos: BlockPos, pipe_type: PipeType) -> u64 {
        if let Some(&id) = self.block_to_network.get(&pos) {
            return id;
        }

        let id = self
            .next_network_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let network = PipeNetwork {
            id,
            pipe_type,
            members: std::collections::HashSet::from([pos]),
            total_transfer_rate: pipe_type.base_transfer_rate(),
        };
        self.networks.insert(id, network);
        self.block_to_network.insert(pos, id);
        id
    }

    pub fn merge_networks(&mut self, id1: u64, id2: u64) {
        if id1 == id2 {
            return;
        }
        if let Some(net2) = self.networks.remove(&id2) {
            if let Some(net1) = self.networks.get_mut(&id1) {
                for pos in net2.members {
                    net1.members.insert(pos);
                    self.block_to_network.insert(pos, id1);
                }
                net1.total_transfer_rate = net1.total_transfer_rate.max(net2.total_transfer_rate);
            }
        }
    }

    pub fn remove_block(&mut self, pos: BlockPos) -> Option<u64> {
        self.block_to_network.remove(&pos).map(|id| {
            if let Some(net) = self.networks.get_mut(&id) {
                net.members.remove(&pos);
                if net.members.is_empty() {
                    self.networks.remove(&id);
                }
            }
            id
        })
    }

    pub fn get_network(&self, id: u64) -> Option<&PipeNetwork> {
        self.networks.get(&id)
    }

    pub fn get_network_mut(&mut self, id: u64) -> Option<&mut PipeNetwork> {
        self.networks.get_mut(&id)
    }

    pub fn all_network_ids(&self) -> Vec<u64> {
        self.networks.keys().copied().collect()
    }
}

/// Settings for the mod
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PipezSettings {
    pub debug_mode: bool,
    pub show_pipe_connections: bool,
    pub log_transfers: bool,
}

/// Thread-local storage for game state
thread_local! {
    static PIPE_MANAGER: std::cell::RefCell<PipeNetworkManager> = std::cell::RefCell::new(PipeNetworkManager::new());
    static SETTINGS: std::cell::RefCell<PipezSettings> = std::cell::RefCell::new(PipezSettings::default());
    static SETTINGS_LOADED: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

/// Read settings from storage
pub fn read_settings(get: impl Fn(&str) -> Option<String>) -> PipezSettings {
    SETTINGS.with(|s| {
        if !SETTINGS_LOADED.get() {
            let settings = PipezSettings {
                debug_mode: get("debug_mode").is_some_and(|v| v.trim() == "true"),
                show_pipe_connections: get("show_pipe_connections")
                    .is_some_and(|v| v.trim() == "true"),
                log_transfers: get("log_transfers").is_some_and(|v| v.trim() == "true"),
            };
            *s.borrow_mut() = settings;
            SETTINGS_LOADED.set(true);
        }
        s.borrow().clone()
    })
}

/// Write settings to storage
pub fn write_settings(settings: &PipezSettings) -> [(&'static str, String); 3] {
    [
        (
            "debug_mode",
            if settings.debug_mode { "true" } else { "false" }.to_string(),
        ),
        (
            "show_pipe_connections",
            if settings.show_pipe_connections {
                "true"
            } else {
                "false"
            }
            .to_string(),
        ),
        (
            "log_transfers",
            if settings.log_transfers {
                "true"
            } else {
                "false"
            }
            .to_string(),
        ),
    ]
}

/// Get current settings
pub fn current_settings() -> PipezSettings {
    SETTINGS.with(|s| s.borrow().clone())
}

/// Update a setting
pub fn update_setting(name: &str, value: bool) {
    SETTINGS.with(|s| {
        let mut settings = s.borrow_mut();
        match name {
            "debug_mode" => settings.debug_mode = value,
            "show_pipe_connections" => settings.show_pipe_connections = value,
            "log_transfers" => settings.log_transfers = value,
            _ => {}
        }
    });
}

/// Main tick function - processes all pipe networks
pub fn tick() {
    PIPE_MANAGER.with(|manager| {
        let mut manager = manager.borrow_mut();
        let network_ids = manager.all_network_ids();
        for network_id in network_ids {
            process_network(&mut manager, network_id);
        }
    });
}

fn process_network(manager: &mut PipeNetworkManager, network_id: u64) {
    let Some(network) = manager.networks.get(&network_id) else {
        return;
    };

    let pipe_type = network.pipe_type;
    let _transfer_rate = network.total_transfer_rate;

    // Collect all member positions
    let members: Vec<BlockPos> = network.members.iter().copied().collect();

    // For each member, try to transfer
    for _pos in members {
        // Process based on pipe type
        match pipe_type {
            PipeType::Item => { /* process_item_transfer(pos, transfer_rate); */ }
            PipeType::Fluid => { /* process_fluid_transfer(pos, transfer_rate); */ }
            PipeType::Energy => { /* process_energy_transfer(pos, transfer_rate); */ }
            PipeType::Gas => { /* process_gas_transfer(pos, transfer_rate); */ }
            PipeType::Universal => {
                // Universal pipes can transfer anything
                // process_item_transfer(pos, transfer_rate);
                // process_fluid_transfer(pos, transfer_rate);
                // process_energy_transfer(pos, transfer_rate);
                // process_gas_transfer(pos, transfer_rate);
            }
        }
    }
}

/// Handle player interaction (wrench, filter tool)
pub fn on_player_interact(
    held_item: &str,
    block_pos: BlockPos,
    is_sneaking: bool,
    _is_right_click: bool,
) {
    if held_item == "pipez:wrench" {
        handle_wrench_interaction(block_pos, is_sneaking);
    } else if held_item == "pipez:filter_destination_tool" {
        handle_filter_tool_interaction(block_pos);
    }
}

fn handle_wrench_interaction(pos: BlockPos, is_sneaking: bool) {
    if is_sneaking {
        // Remove pipe - would call game.break_block(pos)
    } else {
        // Rotate connection or cycle mode
        PIPE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            if let Some(network_id) = manager.block_to_network.get(&pos).copied() {
                if let Some(_network) = manager.networks.get_mut(&network_id) {
                    // Would update connections here
                }
            }
        });
    }
}

fn handle_filter_tool_interaction(_pos: BlockPos) {
    // Open filter GUI or cycle filter mode
}

/// Handle block break
pub fn on_block_break(block_pos: BlockPos) {
    PIPE_MANAGER.with(|manager| {
        let mut manager = manager.borrow_mut();
        manager.remove_block(block_pos);
    });
}

/// Handle block place
pub fn on_block_place(
    block_pos: BlockPos,
    block_id: &str,
    get_block: impl Fn(BlockPos) -> Option<String>,
) {
    if let Some(pipe_type) = PipeType::from_block_id(block_id) {
        PIPE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            let network_id = manager.get_or_create_network(block_pos, pipe_type);

            // Check adjacent pipes and merge networks
            for dir in Direction::ALL {
                let adj_pos = block_pos.offset(dir);
                if let Some(adj_block) = get_block(adj_pos) {
                    if let Some(adj_type) = PipeType::from_block_id(&adj_block) {
                        if pipe_type.can_connect_to(adj_type) {
                            if let Some(adj_network_id) =
                                manager.block_to_network.get(&adj_pos).copied()
                            {
                                if adj_network_id != network_id {
                                    manager.merge_networks(network_id, adj_network_id);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

/// List settings for mod menu
pub fn list_settings() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "debug_mode",
            "Debug Mode",
            "Enable debug logging for pipe networks",
        ),
        (
            "show_pipe_connections",
            "Show Pipe Connections",
            "Render pipe connection overlays",
        ),
        (
            "log_transfers",
            "Log Transfers",
            "Log all item/fluid/energy transfers to console",
        ),
    ]
}

/// Get a setting value
pub fn get_setting(name: &str) -> Option<bool> {
    SETTINGS.with(|settings| {
        let settings = settings.borrow();
        match name {
            "debug_mode" => Some(settings.debug_mode),
            "show_pipe_connections" => Some(settings.show_pipe_connections),
            "log_transfers" => Some(settings.log_transfers),
            _ => None,
        }
    })
}

/// Set a setting value
pub fn set_setting(name: &str, value: bool) -> bool {
    SETTINGS.with(|settings| {
        let mut settings = settings.borrow_mut();
        match name {
            "debug_mode" => {
                settings.debug_mode = value;
                true
            }
            "show_pipe_connections" => {
                settings.show_pipe_connections = value;
                true
            }
            "log_transfers" => {
                settings.log_transfers = value;
                true
            }
            _ => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_type_from_block_id() {
        assert_eq!(
            PipeType::from_block_id("pipez:item_pipe"),
            Some(PipeType::Item)
        );
        assert_eq!(
            PipeType::from_block_id("pipez:fluid_pipe"),
            Some(PipeType::Fluid)
        );
        assert_eq!(
            PipeType::from_block_id("pipez:energy_pipe"),
            Some(PipeType::Energy)
        );
        assert_eq!(
            PipeType::from_block_id("pipez:gas_pipe"),
            Some(PipeType::Gas)
        );
        assert_eq!(
            PipeType::from_block_id("pipez:universal_pipe"),
            Some(PipeType::Universal)
        );
        assert_eq!(PipeType::from_block_id("minecraft:stone"), None);
    }

    #[test]
    fn test_upgrade_tier_multiplier() {
        assert_eq!(UpgradeTier::Basic.multiplier(), 1.5);
        assert_eq!(UpgradeTier::Improved.multiplier(), 2.5);
        assert_eq!(UpgradeTier::Advanced.multiplier(), 4.0);
        assert_eq!(UpgradeTier::Ultimate.multiplier(), 8.0);
        assert_eq!(UpgradeTier::Infinity.multiplier(), 16.0);
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::Up.opposite(), Direction::Down);
        assert_eq!(Direction::Down.opposite(), Direction::Up);
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::West.opposite(), Direction::East);
        assert_eq!(Direction::East.opposite(), Direction::West);
    }

    #[test]
    fn test_block_pos_offset() {
        let pos = BlockPos::new(10, 64, 10);
        assert_eq!(pos.offset(Direction::Up), BlockPos::new(10, 65, 10));
        assert_eq!(pos.offset(Direction::Down), BlockPos::new(10, 63, 10));
        assert_eq!(pos.offset(Direction::North), BlockPos::new(10, 64, 9));
        assert_eq!(pos.offset(Direction::South), BlockPos::new(10, 64, 11));
        assert_eq!(pos.offset(Direction::West), BlockPos::new(9, 64, 10));
        assert_eq!(pos.offset(Direction::East), BlockPos::new(11, 64, 10));
    }

    #[test]
    fn test_pipe_network_manager() {
        let mut manager = PipeNetworkManager::new();
        let pos1 = BlockPos::new(0, 0, 0);
        let pos2 = BlockPos::new(1, 0, 0);

        let id1 = manager.get_or_create_network(pos1, PipeType::Item);
        let id2 = manager.get_or_create_network(pos2, PipeType::Item);

        assert_ne!(id1, id2);

        // Manually merge
        manager.merge_networks(id1, id2);

        // Both should now be in same network
        assert_eq!(manager.block_to_network.get(&pos1), Some(&id1));
        assert_eq!(manager.block_to_network.get(&pos2), Some(&id1));

        let network = manager.networks.get(&id1).unwrap();
        assert_eq!(network.members.len(), 2);
        assert!(network.members.contains(&pos1));
        assert!(network.members.contains(&pos2));
    }

    #[test]
    fn test_settings() {
        SETTINGS.with(|s| *s.borrow_mut() = PipezSettings::default());
        SETTINGS_LOADED.with(|s| s.set(true));

        assert!(set_setting("debug_mode", true));
        assert_eq!(get_setting("debug_mode"), Some(true));

        assert!(set_setting("show_pipe_connections", true));
        assert_eq!(get_setting("show_pipe_connections"), Some(true));

        assert!(set_setting("log_transfers", true));
        assert_eq!(get_setting("log_transfers"), Some(true));

        assert!(!set_setting("invalid_setting", true));
    }

    #[test]
    fn test_pipe_network_data_recalculate() {
        let mut data = PipeNetworkData::default();
        data.pipe_type = PipeType::Item;
        data.upgrades[0] = 1; // 1 basic upgrade
        data.recalculate_transfer_rate();
        // base 64 * (1 + 1.5) = 160
        assert_eq!(data.transfer_rate, 160);

        data.upgrades[1] = 1; // 1 improved upgrade
        data.recalculate_transfer_rate();
        // 64 * (1 + 1.5 + 2.5) = 320
        assert_eq!(data.transfer_rate, 320);
    }

    #[test]
    fn test_read_write_settings() {
        let stored: std::collections::HashMap<String, String> = [
            ("debug_mode".to_string(), "true".to_string()),
            ("show_pipe_connections".to_string(), "false".to_string()),
            ("log_transfers".to_string(), "true".to_string()),
        ]
        .into_iter()
        .collect();

        let get = |key: &str| stored.get(key).cloned();
        let settings = read_settings(get);
        assert!(settings.debug_mode);
        assert!(!settings.show_pipe_connections);
        assert!(settings.log_transfers);

        let written = write_settings(&settings);
        assert_eq!(written[0], ("debug_mode", "true".to_string()));
        assert_eq!(written[1], ("show_pipe_connections", "false".to_string()));
        assert_eq!(written[2], ("log_transfers", "true".to_string()));
    }
}
