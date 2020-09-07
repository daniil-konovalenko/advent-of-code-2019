use std::fs;
use std::str::FromStr;

fn main() {
    let mut program = read_intcode("inputs/02-intcode.txt");
    let answer = brute_force(&mut program, 19690720);
    dbg!(answer);
}

fn brute_force(program: &mut Vec<usize>, expected: usize) -> usize {
    for verb in 0..program.len() {
        for noun in 0..program.len() {
            if run_program(program, noun, verb) == expected {
                return 100 * noun + verb;
            }
        }
    }
    return 0;
}

fn run_program(initial_program: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut program = initial_program.to_vec();

    program[1] = noun;
    program[2] = verb;

    interpret(&mut program);

    program[0]
}

fn get_op(opcode: usize) -> fn(usize, usize) -> usize {
    match opcode {
        1 => |a, b| a + b,
        2 => |a, b| a * b,
        _ => unreachable!(),
    }
}

fn interpret(program: &mut Vec<usize>) {
    let mut current_pos = 0;
    while current_pos < program.len() {
        match program[current_pos] {
            99 => return,
            code @ 1 | code @ 2 => {
                if let [a_pos, b_pos, result_pos] = program[(current_pos + 1)..(current_pos + 4)] {
                    let op = get_op(code);
                    apply(program, a_pos, b_pos, result_pos, op)
                }
            }
            code => panic!(format!("unknown opcode {}", code)),
        }
        current_pos += 4
    }
}

fn apply(
    program: &mut Vec<usize>,
    a_pos: usize,
    b_pos: usize,
    result_pos: usize,
    op: fn(usize, usize) -> usize,
) {
    program[result_pos] = op(program[a_pos], program[b_pos]);
}

fn read_intcode<T: FromStr>(filename: &str) -> Vec<T> {
    let contents = fs::read_to_string(filename).expect("failed to read file ");

    return contents
        .split(',')
        .map(|s| s.parse::<T>())
        .filter_map(Result::ok)
        .collect::<Vec<T>>();
}

#[test]
fn test_interpret() {
    let cases: Vec<(Vec<usize>, Vec<usize>)> = vec![
        (
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ),
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
    ];

    for case in cases {
        let (input, expected_output) = case;

        let mut input = input.to_vec();

        interpret(&mut input);
        assert_eq!(input, expected_output)
    }
}
