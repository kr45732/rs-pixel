use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref LEVELING_EXP: &'static [i64] = &[
        50, 125, 200, 300, 500, 750, 1000, 1500, 2000, 3500, 5000, 7500, 10000, 15000, 20000,
        30000, 50000, 75000, 100_000, 200_000, 300_000, 400_000, 500_000, 600_000, 700_000, 800_000,
        900_000, 1_000_000, 1_100_000, 1_200_000, 1_300_000, 1_400_000, 1_500_000, 1_600_000, 1_700_000, 1_800_000,
        1_900_000, 2_000_000, 2_100_000, 2_200_000, 2_300_000, 2_400_000, 2_500_000, 2_600_000, 2_750_000, 2_900_000,
        3_100_000, 3_400_000, 3_700_000, 4_000_000, 4_300_000, 4_600_000, 4_900_000, 5_200_000, 5_500_000, 5_800_000,
        6_100_000, 6_400_000, 6_700_000, 7_000_000,
    ];
    pub static ref RUNECRAFTING_EXP: &'static [i64] = &[
        50, 100, 125, 160, 200, 250, 315, 400, 500, 625, 785, 1000, 1250, 1600, 2000, 2465, 3125,
        4000, 5000, 6200, 7800, 9800, 12200, 15300, 19050,
    ];
    pub static ref HOTM_EXP: &'static [i64] = &[0, 3000, 9000, 25000, 60000, 100_000, 150_000];
    pub static ref SOCIAL_EXP: &'static [i64] = &[
        50, 100, 150, 250, 500, 750, 1000, 1250, 1500, 2000, 2500, 3000, 3750, 4500, 6000, 8000,
        10000, 12500, 15000, 20000, 25000, 30000, 35000, 40000, 50000,
    ];
    pub static ref CATACOMBS_EXP: &'static [i64] = &[
        50, 75, 110, 160, 230, 330, 470, 670, 950, 1340, 1890, 2665, 3760, 5260, 7380, 10300,
        14400, 20000, 27600, 38000, 52500, 71500, 97000, 132_000, 180_000, 243_000, 328_000, 445_000,
        600_000, 800_000, 1_065_000, 1_410_000, 1_900_000, 2_500_000, 3_300_000, 4_300_000, 5_600_000, 7_200_000,
        9_200_000, 12_000_000, 15_000_000, 19_000_000, 24_000_000, 30_000_000, 38_000_000, 48_000_000, 60_000_000,
        75_000_000, 93_000_000, 116_250_000,
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
        m.insert("wolf", 9);
        m.insert("spider", 9);
        m.insert("zombie", 9);
        m.insert("enderman", 9);
        m.insert("blaze", 9);
        m
    };
    pub static ref ZOMBIE_EXP: &'static [i64] = &[5, 15, 200, 1000, 5000, 20000, 100_000, 400_000, 1_000_000];
    pub static ref SPIDER_EXP: &'static [i64] = &[5, 15, 200, 1000, 5000, 20000, 100_000, 400_000, 1_000_000];
    pub static ref WOLF_EXP: &'static [i64] = &[10, 30, 250, 1500, 5000, 20000, 100_000, 400_000, 1_000_000];
    pub static ref ENDERMAN_EXP: &'static [i64] = &[10, 30, 250, 1500, 5000, 20000, 100_000, 400_000, 1_000_000];
    pub static ref BLAZE_EXP: &'static [i64] = &[10, 30, 250, 1500, 5000, 20000, 100_000, 400_000, 1_000_000];
    pub static ref PET_RARITY_OFFSET: HashMap<&'static str, i64> = {
        let mut m = HashMap::new();
        m.insert("COMMON", 0);
        m.insert("UNCOMMON",6);
        m.insert("RARE", 11);
        m.insert("EPIC", 16);
        m.insert("LEGENDARY", 20);
        m.insert("MYTHIC", 20);
        m
    };
    pub static ref PET_EXP: &'static [i64] = &[
        100, 110, 120, 130, 145, 160, 175, 190, 210, 230, 250, 275, 300, 330, 360, 400, 440, 490, 540,
        600, 660, 730, 800, 880, 960, 1050, 1150, 1260, 1380, 1510, 1650, 1800, 1960, 2130, 2310, 2500,
        2700, 2920, 3160, 3420, 3700, 4000, 4350, 4750, 5200, 5700, 6300, 7000, 7800, 8700, 9700,
        10800, 12000, 13300, 14700, 16200, 17800, 19500, 21300, 23200, 25200, 27400, 29800, 32400,
        35200, 38200, 41400, 44800, 48400, 52200, 56200, 60400, 64800, 69400, 74200, 79200, 84700,
        90700, 97200, 104_200, 111_700, 119_700, 128_200, 137_200, 146_700, 156_700, 167_700, 179_700, 192_700,
        206_700, 221_700, 237_700, 254_700, 272_700, 291_700, 311_700, 333_700, 357_700, 383_700, 411_700, 441_700,
        476_700, 516_700, 561_700, 611_700, 666_700, 726_700, 791_700, 861_700, 936_700, 1_016_700, 1_101_700,
        1_191_700, 1_286_700, 1_386_700, 1_496_700, 1_616_700, 1_746_700, 1_886_700,
    ];

    // Other
    pub static ref CRAFTED_MINIONS_TO_SLOTS: &'static [i64] = &[
        0, 5, 15, 30, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 350, 400, 450, 500, 550, 600,
    ];
}
