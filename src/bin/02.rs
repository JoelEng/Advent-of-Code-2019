use helpers;

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let opcodes = helpers::to_opcode(input);
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
    let mut pos = 0;
    while let Some(inc) = instruction(&mut opcodes, pos) {
        pos += inc;
    }
    opcodes[0]
}

fn instruction(opcodes: &mut Vec<usize>, pos: usize) -> Option<usize> {
    let mut a = vec![0; 4];
    a.clone_from_slice(&opcodes[pos..pos + 4]);
    match a[0] {
        1 => {
            opcodes[a[3]] = opcodes[a[1]] + opcodes[a[2]];
            Some(4)
        }
        2 => {
            opcodes[a[3]] = opcodes[a[1]] * opcodes[a[2]];
            Some(4)
        }
        99 => None,
        _ => None,
    }
}
