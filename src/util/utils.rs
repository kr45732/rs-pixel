const BASE: f64 = 10000.0;
const GROWTH: f64 = 2500.0;
const HALF_GROWTH: f64 = 0.5 * GROWTH;
const REVERSE_PQ_PREFIX: f64 = -(BASE - 0.5 * GROWTH) / GROWTH;
const REVERSE_CONST: f64 = REVERSE_PQ_PREFIX * REVERSE_PQ_PREFIX;
const GROWTH_DIVIDES_2: f64 = 2.0 / GROWTH;

pub fn get_total_exp_to_full_level(level: f64) -> f64 {
    (HALF_GROWTH * (level - 2.0) + BASE) * (level - 1.0)
}

pub fn get_exact_level(exp: f64) -> f64 {
    get_level(exp) + get_percentage_to_next_level(exp)
}

pub fn get_level(exp: f64) -> f64 {
    if exp < 0.0 {
        1.0
    } else {
        (1.0 + REVERSE_PQ_PREFIX + (REVERSE_CONST + GROWTH_DIVIDES_2 * exp).sqrt()).floor()
    }
}

pub fn get_percentage_to_next_level(exp: f64) -> f64 {
    let lvl = get_level(exp);
    let x0 = get_total_exp_to_level(lvl);
    (exp - x0) / (get_total_exp_to_level(lvl + 1.0) - x0)
}

pub fn get_total_exp_to_level(level: f64) -> f64 {
    let lvl = level.floor();
    let x0 = get_total_exp_to_full_level(lvl);
    if level == lvl {
        x0
    } else {
        (get_total_exp_to_full_level(lvl + 1.0) - x0) * (level % 1.0) + x0
    }
}
