use anyhow::Result;
use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::zh_cn::Name;
use fake::{Dummy, Fake, Faker};
use nanoid::nanoid;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
// generate 10_000 users and run the in a tx, repeat 500 times

#[derive(Debug, Clone, Dummy, Serialize, Deserialize, PartialEq, Eq)]
enum Gender {
    Female,
    Male,
    Unknown,
}
#[derive(Debug, Clone, Dummy, Serialize, Deserialize)]
struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,

    #[dummy(faker = "Name()")]
    name: String,

    gender: Gender,

    #[dummy(faker = "DateTimeBetween(before(365*5), before(90))")]
    created_at: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(before(30), now())")]
    last_visited_at: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_watched_at: DateTime<Utc>,

    #[dummy(faker = "IntList(50, 10000, 100000)")]
    recent_watched: Vec<i32>,

    #[dummy(faker = "IntList(50, 20000, 100000)")]
    viewed_but_not_started: Vec<i32>,

    #[dummy(faker = "IntList(50, 30000, 100000)")]
    started_but_not_finished: Vec<i32>,

    #[dummy(faker = "IntList(50, 40000, 100000)")]
    finished: Vec<i32>,

    #[dummy(faker = "DateTimeBetween(before(45), now())")]
    last_email_notification: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(before(15), now())")]
    last_in_app_notification: DateTime<Utc>,

    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    last_sms_notification: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let user: UserStat = Faker.fake();

    println!("{:?}", user);
    Ok(())
}

async fn raw_insert(user: UserStat) -> Result<()> {
    Ok(())
}

fn list_to_string(list: Vec<i32>) -> String {
    todo!()
}

#[allow(dead_code)]
async fn bulk_insert(users: Vec<UserStat>) -> Result<()> {
    Ok(())
}

fn before(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}

fn now() -> DateTime<Utc> {
    Utc::now()
}

impl Hash for UserStat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

struct IntList(pub i32, pub i32, pub i32);

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &IntList, rng: &mut R) -> Self {
        let (max, start, len) = (config.0, config.1, config.2);
        let size = rng.gen_range(0..max);
        (0..size)
            .map(|_| rng.gen_range(start..start + len))
            .collect()
    }
}

struct UniqueEmail;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &UniqueEmail, rng: &mut R) -> Self {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &ALPHABET);
        let at = email.find('@').unwrap();
        format!("{}.{}{}", &email[..at], id, &email[at..])
    }
}
