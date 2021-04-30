const STARTING_MISSILES: i32  = 8;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
