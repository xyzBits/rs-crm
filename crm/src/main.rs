pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/crm.rs"));
}

fn main() {
    let user = pb::User::default();
    println!("{:?}", user);
    println!("Hello, world!");
}
