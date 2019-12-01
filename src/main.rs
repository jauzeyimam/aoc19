use math::round;

fn main() {
    println!("1: {}", solve_one())
}

fn solve_one() -> u32 {
    static NUMBERS: &'static [u32] = &[
        66690, 86239, 75191, 140364, 95979, 106923, 95229, 123571, 84764, 89444, 98107, 89062,
        109369, 146067, 124760, 76900, 139198, 111441, 74046, 84920, 54397, 143807, 121654, 93863,
        73909, 104121, 58485, 119084, 126227, 142078, 79820, 132617, 108430, 98032, 107434, 127307,
        105619, 57741, 53468, 63301, 137970, 136780, 80897, 133205, 79159, 89124, 94477, 56714,
        143704, 122097, 117335, 108246, 75507, 101459, 101162, 146197, 121884, 66217, 57074,
        142903, 140951, 64883, 124556, 67382, 142407, 121778, 57933, 94599, 87426, 143758, 64043,
        65678, 90137, 61090, 77315, 102383, 146607, 139290, 85394, 149787, 125611, 106405, 91561,
        135739, 54845, 68782, 111175, 61011, 125658, 70751, 85607, 75458, 75419, 124311, 66022,
        122784, 129018, 54901, 73788, 108240,
    ];

    let mut sum: u32 = 0;

    for number in NUMBERS {
        let feul = calc_feul(*number);
        sum += feul;
    }

    sum
}

fn calc_feul(mass: u32) -> u32 {
    let divide_by_three = mass as f32 / 3.0;
    let floored = round::floor(divide_by_three as f64, 0) as i32;
    let subtract_two = floored - 2;

    // println!("subtract_two: {}", subtract_two);

    if subtract_two <= 0 {
        return 0;
    }

    let result_feul = calc_feul(subtract_two as u32);
    let result = subtract_two as u32 + result_feul;

    result
}