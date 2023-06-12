pub fn init(input: &str) -> Vec<usize> {
    input.split(",").filter_map(|s| s.parse().ok()).collect()
}

pub fn run(opcodes: &mut Vec<usize>) {
    let mut pos = 0;
    while let Some(inc) = instruction(opcodes, pos) {
        pos += inc;
    }
}

fn instruction(opcodes: &mut Vec<usize>, pos: usize) -> Option<usize> {
    let size = match opcodes[pos] {
        1 => 4,
        2 => 4,
        99 | _ => return None,
    };
    let mut i = vec![0; size];
    i.clone_from_slice(&opcodes[pos..pos + size]);
    match i[0] {
        1 => {
            opcodes[i[3]] = opcodes[i[1]] + opcodes[i[2]];
        }
        2 => {
            opcodes[i[3]] = opcodes[i[1]] * opcodes[i[2]];
        }
        _ => (),
    }
    Some(size)
}
