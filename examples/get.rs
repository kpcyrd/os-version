fn main() {
    let version = os_version::detect();
    println!("{:?}", version);
}
