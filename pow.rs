fn binfmt(num: u64) -> Vec<u32> {
    let bin = format!("{num:b}");
    let mut int_vec: Vec<u32> = vec![];
    let mut multi = 1;
    for c in bin.chars().rev() {
        if c == '1' {
            int_vec.push(multi);
        }
        multi *= 2;
    }
    return int_vec;
}

fn pow(num: u64, exp: u64, modulo: u64) -> u64 {
    let mut result: u64 = 1;
    let vec_exps = binfmt(exp);
    for i in vec_exps {
        result *= num.pow(i) % modulo;
    }
    return result;
}
