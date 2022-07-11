use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    // Leveling data taken from NotEnoughUpdates-REPO (https://github.com/NotEnoughUpdates/NotEnoughUpdates-REPO/blob/master/constants/leveling.json)

    pub static ref LEVELING_XP: &'static [i64] = &[
        50, 125, 200, 300, 500, 750, 1000, 1500, 2000, 3500, 5000, 7500, 10000, 15000, 20000,
        30000, 50000, 75000, 100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000,
        900000, 1000000, 1100000, 1200000, 1300000, 1400000, 1500000, 1600000, 1700000, 1800000,
        1900000, 2000000, 2100000, 2200000, 2300000, 2400000, 2500000, 2600000, 2750000, 2900000,
        3100000, 3400000, 3700000, 4000000, 4300000, 4600000, 4900000, 5200000, 5500000, 5800000,
        6100000, 6400000, 6700000, 7000000,
    ];
    pub static ref RUNECRAFTING_XP: &'static [i64] = &[
        50, 100, 125, 160, 200, 250, 315, 400, 500, 625, 785, 1000, 1250, 1600, 2000, 2465, 3125,
        4000, 5000, 6200, 7800, 9800, 12200, 15300, 19050,
    ];
    pub static ref HOTM_XP: &'static [i64] = &[0, 3000, 9000, 25000, 60000, 100000, 150000];
    pub static ref SOCIAL_XP: &'static [i64] = &[
        50, 100, 150, 250, 500, 750, 1000, 1250, 1500, 2000, 2500, 3000, 3750, 4500, 6000, 8000,
        10000, 12500, 15000, 20000, 25000, 30000, 35000, 40000, 50000,
    ];
    pub static ref CATACOMBS_XP: &'static [i64] = &[
        50, 75, 110, 160, 230, 330, 470, 670, 950, 1340, 1890, 2665, 3760, 5260, 7380, 10300,
        14400, 20000, 27600, 38000, 52500, 71500, 97000, 132000, 180000, 243000, 328000, 445000,
        600000, 800000, 1065000, 1410000, 1900000, 2500000, 3300000, 4300000, 5600000, 7200000,
        9200000, 12000000, 15000000, 19000000, 24000000, 30000000, 38000000, 48000000, 60000000,
        75000000, 93000000, 116250000,
    ];
    pub static ref LEVELING_CAPS: HashMap<&'static str, i64> = {
        let mut m = HashMap::new();
        m.insert("taming", 50);
        m.insert("mining", 60);
        m.insert("foraging", 50);
        m.insert("enchanting", 60);
        m.insert("carpentry", 50);
        m.insert("farming", 50);
        m.insert("combat", 60);
        m.insert("fishing", 50);
        m.insert("alchemy", 50);
        m.insert("runecrafting", 25);
        m.insert("catacombs", 50);
        m.insert("hotm", 7);
        m.insert("social", 25);
        m
    };
}
