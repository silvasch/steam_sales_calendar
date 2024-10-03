use steam_sales_calendar::Sale;

fn main() {
    steam_sales_calendar::make_calendar(&[
        Sale::new(
            "Tower Defense Fest",
            "2024-07-29 17:00:00 UTC".parse().unwrap(),
            "2024-08-05 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Fighting Games Fest",
            "2024-08-05 17:00:00 UTC".parse().unwrap(),
            "2024-08-12 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Rythm Fest",
            "2024-08-19 17:00:00 UTC".parse().unwrap(),
            "2024-08-26 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Space Exploration Fest",
            "2024-09-02 17:00:00 UTC".parse().unwrap(),
            "2024-09-09 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Planes, Trains, and Automobiles Fest",
            "2024-09-16 17:00:00 UTC".parse().unwrap(),
            "2024-09-23 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Turn-Based RPG Fest",
            "2024-09-30 17:00:00 UTC".parse().unwrap(),
            "2024-10-07 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Next Fest: October Edition",
            "2024-10-14 17:00:00 UTC".parse().unwrap(),
            "2024-10-21 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Scream Fest",
            "2024-10-28 17:00:00 UTC".parse().unwrap(),
            "2024-11-04 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Cooking Fest",
            "2024-11-11 17:00:00 UTC".parse().unwrap(),
            "2024-11-18 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Autumn Sale",
            "2024-11-27 17:00:00 UTC".parse().unwrap(),
            "2024-12-04 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Winter Sale",
            "2024-12-19 17:00:00 UTC".parse().unwrap(),
            "2025-01-02 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Real-Time Strategy Fest",
            "2025-01-20 17:00:00 UTC".parse().unwrap(),
            "2025-01-27 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Idler Fest",
            "2025-02-03 17:00:00 UTC".parse().unwrap(),
            "2025-02-10 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Couch Co-Op Fest",
            "2025-02-10 17:00:00 UTC".parse().unwrap(),
            "2025-02-17 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Next Fest: February Edition",
            "2025-02-24 17:00:00 UTC".parse().unwrap(),
            "2025-03-03 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Visual Novel Fest",
            "2025-03-03 17:00:00 UTC".parse().unwrap(),
            "2025-03-10 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Spring Fest",
            "2025-03-13 17:00:00 UTC".parse().unwrap(),
            "2025-03-20 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "City Builder & Colony Sim Fest",
            "2025-03-24 17:00:00 UTC".parse().unwrap(),
            "2025-03-31 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Sokoban Fest",
            "2025-04-21 17:00:00 UTC".parse().unwrap(),
            "2025-04-28 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Wargames Fest",
            "2025-04-28 17:00:00 UTC".parse().unwrap(),
            "2025-05-05 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Creature Collector Fest",
            "2025-05-12 17:00:00 UTC".parse().unwrap(),
            "2025-05-19 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Zombies vs. Vampires Fest",
            "2025-05-26 17:00:00 UTC".parse().unwrap(),
            "2025-06-02 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Next Fest: June Edition",
            "2025-06-09 17:00:00 UTC".parse().unwrap(),
            "2025-06-16 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Fishing Fest",
            "2025-06-16 17:00:00 UTC".parse().unwrap(),
            "2025-06-23 17:00:00 UTC".parse().unwrap(),
        ),
        Sale::new(
            "Steam Summer Sale",
            "2025-06-26 17:00:00 UTC".parse().unwrap(),
            "2025-07-10 17:00:00 UTC".parse().unwrap(),
        ),
    ]);
}
