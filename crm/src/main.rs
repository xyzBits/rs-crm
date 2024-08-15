use crm::pb::User;

fn main() {
    let user = User::new(1, "bitch", "bitch@gmail.com");

    println!("{:?}", user);
}
