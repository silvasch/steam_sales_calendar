use crate::{sales, Sale};

pub fn get_sales<'a>() -> [Sale<'a>; 25] {
    sales! {
        "Tower Defense Fest": 2024-7-29 -> 2024-8-5,
        "Fighting Games Fest": 2024-8-5 -> 2024-8-12,
        "Rythm Fest": 2024-8-19 -> 2024-8-26,
        "Space Exploration Fest": 2024-9-2 -> 2024-9-9,
        "Planes: Trains: and Automobiles Fest": 2024-9-16 -> 2024-9-23,
        "Turn-Based RPG Fest": 2024-9-30 -> 2024-10-7,
        "Steam Next Fest: October Edition": 2024-10-14 -> 2024-10-21,
        "Scream Fest": 2024-10-28 -> 2024-11-4,
        "Cooking Fest": 2024-11-11 -> 2024-11-18,
        "Steam Autumn Sale": 2024-11-27 -> 2024-12-4,
        "Steam Winter Sale": 2024-12-19 -> 2025-1-2,
        "Real-Time Strategy Fest": 2025-1-20 -> 2025-1-27,
        "Idler Fest": 2025-2-3 -> 2025-2-10,
        "Couch Co-Op Fest": 2025-2-10 -> 2025-2-17,
        "Steam Next Fest: February Edition": 2025-2-24 -> 2025-3-3,
        "Visual Novel Fest": 2025-3-3 -> 2025-3-10,
        "Steam Spring Fest": 2025-3-13 -> 2025-3-20,
        "City Builder & Colony Sim Fest": 2025-3-24 -> 2025-3-31,
        "Sokoban Fest": 2025-4-21 -> 2025-4-28,
        "Wargames Fest": 2025-4-28 -> 2025-5-5,
        "Creature Collector Fest": 2025-5-12 -> 2025-5-19,
        "Zombies vs. Vampires Fest": 2025-5-26 -> 2025-6-2,
        "Steam Next Fest: June Edition": 2025-6-9 -> 2025-6-16,
        "Fishing Fest": 2025-6-16 -> 2025-6-23,
        "Steam Summer Sale": 2025-6-26 -> 2025-7-10,
    }
}
