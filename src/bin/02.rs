use helpers::opcode;

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let opcodes = opcode::init(input);
    let mut p2 = 0;
    for n in 0..=99 {
        for v in 0..=99 {
            if run(n, v, opcodes.clone()) == 19690720 {
                p2 = 100 * n + v;
            }
        }
    }

    (run(12, 2, opcodes.clone()), p2)
}

fn run(noun: usize, verb: usize, mut opcodes: Vec<usize>) -> usize {
    opcodes[1] = noun;
    opcodes[2] = verb;
    opcode::run(&mut opcodes);
    opcodes[0]
}
