use super::*;

#[derive(Default, Clone, Copy)]
pub enum EventKey {
    #[default]
    None,
    Combat,
    Energy,
    Mining,
}

pub struct GridEvents {
    pub combat: Vec<EventInfo<CombatAction>>,
    pub energy: Vec<EventInfo<EnergyAction>>,
    pub mining: Vec<EventInfo<MiningAction>>,
}

#[derive(Default, Clone)]
pub struct EventInfo<T> {
    pub data: EventData,
    pub action: T,
}

#[derive(Default, Clone)]
pub struct EventData {
    pub key: EventKey,
    pub enter: bool,
    pub title: String,
    pub descr: String,
}

#[derive(Default, Clone)]
pub struct EventAction {
    pub enter: String,
    pub leave: String,
}

#[derive(Default, Clone, Copy)]
pub struct CombatAction {
    pub is_large: bool,
    pub enemies: u16,
}

#[derive(Default, Clone, Copy)]
pub struct EnergyAction {
    pub is_large: bool,
    pub energy: u16,
}

#[derive(Default, Clone, Copy)]
pub struct MiningAction {
    pub is_large: bool,
    pub material: u16,
}

impl GridEvents {
    pub fn load_data(app: &mut App) {
        let mut event_data = GridEvents {
            combat: Vec::new(),
            energy: Vec::new(),
            mining: Vec::new(),
        };

        event_data.combat.push(EventInfo {
            data: EventData {
                key: EventKey::Combat,
                enter: false,
                title: "Empty Space".to_string(),
                descr: "An empty section of space.".to_string(),
            },
            action: CombatAction {
                is_large: false,
                enemies: 0,
            },
        });
        event_data.combat.push(EventInfo {
            data: EventData {
                key: EventKey::Combat,
                enter: true,
                title: "Small Amboush".to_string(),
                descr: "A small amboush.".to_string(),
            },
            action: CombatAction {
                is_large: false,
                enemies: 4,
            },
        });
        event_data.combat.push(EventInfo {
            data: EventData {
                key: EventKey::Combat,
                enter: true,
                title: "Big Amboush".to_string(),
                descr: "A big amboush.".to_string(),
            },
            action: CombatAction {
                is_large: true,
                enemies: 8,
            },
        });

        event_data.energy.push(EventInfo {
            data: EventData {
                key: EventKey::Energy,
                enter: false,
                title: "Dimm Star".to_string(),
                descr: "This star bearly puts out light.".to_string(),
            },
            action: EnergyAction {
                is_large: false,
                energy: 10,
            },
        });
        event_data.energy.push(EventInfo {
            data: EventData {
                key: EventKey::Energy,
                enter: true,
                title: "Small Star".to_string(),
                descr: "A small star.".to_string(),
            },
            action: EnergyAction {
                is_large: false,
                energy: 30,
            },
        });
        event_data.energy.push(EventInfo {
            data: EventData {
                key: EventKey::Energy,
                enter: true,
                title: "Big Star".to_string(),
                descr: "A big star.".to_string(),
            },
            action: EnergyAction {
                is_large: true,
                energy: 60,
            },
        });

        event_data.mining.push(EventInfo {
            data: EventData {
                key: EventKey::Mining,
                enter: false,
                title: "Empty Belt".to_string(),
                descr: "Seems there is nothing but dust left.".to_string(),
            },
            action: MiningAction {
                is_large: false,
                material: 10,
            },
        });
        event_data.mining.push(EventInfo {
            data: EventData {
                key: EventKey::Mining,
                enter: true,
                title: "Small Belt".to_string(),
                descr: "A small astroid belt.".to_string(),
            },
            action: MiningAction {
                is_large: false,
                material: 25,
            },
        });
        event_data.mining.push(EventInfo {
            data: EventData {
                key: EventKey::Mining,
                enter: true,
                title: "Big Belt".to_string(),
                descr: "A big astroid belt.".to_string(),
            },
            action: MiningAction {
                is_large: true,
                material: 50,
            },
        });

        app.insert_resource(event_data);
    }

    pub fn get_actions(key: EventKey) -> EventAction {
        match key {
            EventKey::None => EventAction {
                enter: "enter".to_string(),
                leave: "leave".to_string(),
            },
            EventKey::Combat => EventAction {
                enter: "attack".to_string(),
                leave: "flee".to_string(),
            },
            EventKey::Energy => EventAction {
                enter: "harvest".to_string(),
                leave: "ignore".to_string(),
            },
            EventKey::Mining => EventAction {
                enter: "explore".to_string(),
                leave: "leave".to_string(),
            },
        }
    }

    pub fn roll_combat_table(&self, seed: i64) -> EventInfo<CombatAction> {
        let mut rng = Shift64::new(seed);
        if rng.i32(256) > 92 {
            let idx = rng.usize(self.combat.len()) + 1;
            return self.combat[idx].clone();
        }
        self.combat[0].clone()
    }

    pub fn roll_energy_table(&self, seed: i64) -> EventInfo<EnergyAction> {
        let mut rng = Shift64::new(seed);
        if rng.i32(256) > 48 {
            let idx = rng.usize(self.energy.len() - 1) + 1;
            return self.energy[idx].clone();
        }
        self.energy[0].clone()
    }

    pub fn roll_mining_table(&self, seed: i64) -> EventInfo<MiningAction> {
        let mut rng = Shift64::new(seed);
        if rng.i32(256) > 24 {
            let idx = rng.usize(self.mining.len() - 1) + 1;
            return self.mining[idx].clone();
        }
        self.mining[0].clone()
    }
}

impl EventData {
    pub fn default() -> EventData {
        EventData {
            key: EventKey::None,
            enter: false,
            title: "Nothing".to_string(),
            descr: "Nothing to see, nothing to do here.".to_string(),
        }
    }
}
