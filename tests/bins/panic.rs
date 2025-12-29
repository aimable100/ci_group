fn main() {
    let _g = ci_group::open("Panic Group");
    panic!("intentional");
}

