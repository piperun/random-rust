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

fn modexp(mut base: u64, mut exp: u64, modular: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modular as u64;
        }
        base = (base * base) % modular as u64;
        exp = exp / 2;
    }
    result as u64
}
