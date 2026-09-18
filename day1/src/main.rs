fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/input1.txt");
    let input = std::fs::read_to_string(path).expect("could not read input.txt");
    let test_cases = vec![input.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>()];
    let mut result: Vec<i32> = vec![];
    for i in test_cases.iter(){
        result.push(rotate_dial(i.clone()));
    }

    println!("Answer is {:?}", result);
}

fn rotate_dial(mut rotations: Vec<&str>) -> i32{
    // two components to rotation: R32, L64
    // i: direction(R|L) ii: number of rotations in direction(30, 2, 4)
    let max_point: i32 = 100;
    let mut current_point: i32 = 50;
    let mut result = 0;
    for i in rotations.iter_mut(){
        let direction: char = i.chars().next().unwrap();
        let num_of_rotation: i32 = i[1..].parse().unwrap();

        // clicks until the pointer next lands on 0 (starting at 0 does not count)
        let first_zero = if current_point == 0 {
            max_point
        } else if direction == 'L' {
            current_point
        } else {
            max_point - current_point
        };

        let clicks = if num_of_rotation >= first_zero {
            1 + (num_of_rotation - first_zero) / max_point
        } else {
            0
        };

        current_point = if direction == 'L' {
            (current_point - num_of_rotation).rem_euclid(max_point)
        } else {
            (current_point + num_of_rotation).rem_euclid(max_point)
        };

        result += clicks;
        println!("{:?} -> {:?}; click: {:?}", i, current_point, clicks);
    }
    result
}
