use clap::Parser;
use serde::Deserialize;
use std::fs::File;
use thing_ranker::account::get_account_by_email;
use thing_ranker::app::{AppState, Config, init_app};
use thing_ranker::category::create_category_inner;
use thing_ranker::rank::create_rank_inner;
use thing_ranker::thing::create_thing_inner;

#[derive(Parser, Debug)]
struct Args {
    /// Email of account that will seed the data
    #[arg(long, short, required = true)]
    email: String,
}

/// Record in thing csv
#[derive(Debug, Deserialize)]
struct ThingRecord {
    name: String,
    image: String,
}

/// Record in category csv
#[derive(Debug, Deserialize)]
struct CategoryRecord {
    name: String,
    image: String,
}

/// Record in rank csv
#[derive(Debug, Deserialize)]
struct RankRecord {
    thing: String,
    category: String,
}

/// CLI utility that prints out auth tokens for accounts specified in the
/// `account_settings` section of config.yml file(s).
/// This is useful for local manual testing via an HTTP client like Postman or Bruno.
#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Initializes app
    let config = Config::load();
    let state = init_app(&config).await;

    // Gets account that will seed the data
    let account = get_account_by_email(&args.email, &state.pool)
        .await
        .unwrap();
    let account = match account {
        Some(account) => account,
        None => {
            eprintln!("Could not find account with email {}", args.email);
            return;
        }
    };

    // Seeds things
    println!("----- Seeding Things -----");
    seed_things(account.id, &state).await;

    // Seeds categories
    println!("----- Seeding Categories -----");
    seed_categories(account.id, &state).await;

    // Seeds ranks
    println!("----- Seeds Ranks -----");
    seed_ranks(account.id, &state).await;
}

async fn seed_things(account_id: i32, state: &AppState) {
    const PATH: &str = "seed/things.csv";
    let reader = match File::open(PATH) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Failed to read file {PATH}: {err}");
            return;
        }
    };
    let mut reader = csv::Reader::from_reader(reader);
    for record in reader.deserialize() {
        let record: ThingRecord = record.unwrap();
        let image_bytes = match tokio::fs::read(&record.image).await {
            Ok(image_bytes) => image_bytes,
            Err(err) => {
                eprintln!("Failed to read file {}: {}", record.image, err);
                continue;
            }
        };
        match create_thing_inner(state, account_id, &record.name, &image_bytes).await {
            Ok(thing) => println!("Created thing {}", thing.name),
            Err(err) => eprintln!("Failed to create thing {}: {}", record.name, err),
        }
    }
}

async fn seed_categories(account_id: i32, state: &AppState) {
    const PATH: &str = "seed/categories.csv";
    let reader = match File::open(PATH) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Failed to read file {PATH}: {err}");
            return;
        }
    };
    let mut reader = csv::Reader::from_reader(reader);
    for record in reader.deserialize() {
        let record: CategoryRecord = record.unwrap();
        let image_bytes = match tokio::fs::read(&record.image).await {
            Ok(image_bytes) => image_bytes,
            Err(err) => {
                eprintln!("Failed to read file {}: {}", record.image, err);
                continue;
            }
        };
        match create_category_inner(state, account_id, &record.name, &image_bytes).await {
            Ok(category) => println!("Created category {}", category.name),
            Err(err) => eprintln!("Failed to create category {}: {}", record.name, err),
        }
    }
}

async fn seed_ranks(account_id: i32, state: &AppState) {
    const PATH: &str = "seed/ranks.csv";
    let reader = match File::open(PATH) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Failed to read file {PATH}: {err}");
            return;
        }
    };
    let mut reader = csv::Reader::from_reader(reader);
    for record in reader.deserialize() {
        let record: RankRecord = record.unwrap();
        let thing_id: Result<i32, _> = sqlx::query_scalar("SELECT id FROM thing WHERE name=$1")
            .bind(&record.thing)
            .fetch_one(&state.pool)
            .await;
        let thing_id = match thing_id {
            Ok(thing_id) => thing_id,
            Err(err) => {
                eprintln!(
                    "Failed to fetch thing with name '{}': {}",
                    record.thing, err
                );
                continue;
            }
        };
        let category_id: Result<i32, _> =
            sqlx::query_scalar("SELECT id FROM category WHERE name=$1")
                .bind(&record.category)
                .fetch_one(&state.pool)
                .await;
        let category_id = match category_id {
            Ok(category_id) => category_id,
            Err(err) => {
                eprintln!(
                    "Failed to fetch category with name '{}': {}",
                    record.category, err
                );
                continue;
            }
        };
        match create_rank_inner(state, account_id, thing_id, category_id).await {
            Ok(_) => println!(
                "Created rank for thing '{}' and category '{}'",
                record.thing, record.category
            ),
            Err(err) => eprintln!(
                "Failed to create rank for thing '{}' and category '{}': {}",
                record.thing, record.category, err
            ),
        }
    }
}
