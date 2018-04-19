extern crate time;
extern crate sha1;
extern crate bincode;

extern crate serde;
#[macro_use] extern crate serde_derive;

use std::io::stdin;

#[derive(Serialize)]
struct HashContent {
    timestamp: i64,
    data: i32,
}

#[derive(Serialize)]
struct Block {
    content: HashContent,
    previous: String,
    current: String,
}

impl Block {

    /// One block constructor. Creates the block from the given data and previous digest.
    ///
    /// Args:
    ///
    /// `data` - the data of the block
    /// `previous` - the digest of the previous block (empty if genesis)
    ///
    /// Returns:
    ///
    /// genesis block
    fn new(
        data: i32,
        previous: String,
    ) -> Block {

        let content = HashContent {
            timestamp: get_current_timestamp(),
            data: data,
        };

        let hash = generate_hash(&content);

        Block {
            content: content,
            previous: previous,
            current: hash,
        }
    }

    /// Getter of the current block hash digest.
    ///
    /// Returns:
    ///
    /// current block digest as string
    fn get_current(&self) -> &str {
        &self.current
    }
}

/// Generates the digest of a given hash content.
///
/// Args:
///
/// `content` - the content to process
///
/// Returns:
///
/// the hash digest as a string
fn generate_hash(content: &HashContent) -> String {

    let bytes = bincode::serialize(&content).unwrap();
    sha1::Sha1::from(bytes).hexdigest()
}

/// Refactor the current timestamp generation.
///
/// Returns:
///
/// the current timestamp
fn get_current_timestamp() -> i64 {
    time::now_utc().to_timespec().sec
}

/// Handles user input and returns that input as a string.
///
/// Returns:
///
/// user input as string
fn get_input() -> String {

    let mut input = String::new();
    stdin().read_line(&mut input).expect("cannot read input");

    input
}

fn main() {

    let genesis = Block::new(0, String::new());
    let mut chain: Vec<Block> = vec![genesis];

    println!("Genesis block has been generated.");

    loop {

        println!("\nChoices:");
        println!("1. Add a block");
        println!("2. Update blockchain");

        let input = get_input();
        let choice = input.as_bytes()[0];

        const ADD_BLOCK_CHOICE: u8 = 0x31;

        if choice == ADD_BLOCK_CHOICE {

            println!("Data of the block:");

            let input = get_input();
            let data: i32 = input.trim().parse().unwrap();

            let current_digest = chain.last()
                .unwrap()
                .get_current()
                .to_string();

            let block = Block::new(data, current_digest.clone());

            println!("One block has been added to the ledger.");
            println!("Current block digest: {}", current_digest);

            chain.push(block);
        }
    }
}
