use arrayref::array_ref;
use dungeon::{bsp, draw, roomscorridors};

use clap::{App, Arg};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sha2::{Digest, Sha256};

use draw::draw;

use bsp::BspLevel;
use roomscorridors::RoomsCorridors;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

enum Algorithm {
    Bsp,
    Rooms,
}

fn main() {
    // config:
    // hash (pass hash directly)
    // text (hashed + used)
    // width
    // height
    // max rooms
    // room size
    let matches = App::new("Dungeon")
        .version("3.0")
        .author("James Baum <@whostolemyhat>")
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .takes_value(true)
                .help("A string to hash and use as a seed"),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .help("An existing seed. Must be 32 characters"),
        )
        .arg(
            Arg::with_name("algo")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["rooms", "bsp"])
                .default_value("rooms")
                .help("The type of procedural algorithm to use"),
        )
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json")
                .help("If set, displays serialised JSON output"),
        )
        .arg(
            Arg::with_name("draw")
                .short("d")
                .long("draw")
                .help("If set, creates a png representation"),
        )
        .arg(
            Arg::with_name("csv")
                .short("c")
                .long("csv")
                .help("Output board in CSV format"),
        )
        .arg(
            Arg::with_name("walls")
                .short("w")
                .long("walls")
                .help("Add wall tile around rooms"),
        )
        .arg(
            Arg::with_name("height")
                .short("y")
                .takes_value(true)
                .default_value("40")
                .long("height")
                .help("Height of the level"),
        )
        .arg(
            Arg::with_name("width")
                .short("x")
                .long("width")
                .takes_value(true)
                .default_value("48")
                .help("Width of the level"),
        )
        .arg(
            Arg::with_name("minroomwidth")
                .short("m")
                .long("minroomwidth")
                .takes_value(true)
                .default_value("4")
                .help("Minimum width of rooms"),
        )
        .arg(
            Arg::with_name("minroomheight")
                .short("n")
                .long("minroomheight")
                .takes_value(true)
                .default_value("5")
                .help("Minimum height of rooms"),
        )
        .get_matches();

    let board_width = matches
        .value_of("width")
        .expect("Width not set")
        .parse::<i32>()
        .expect("Error parsing width");
    let board_height = matches
        .value_of("height")
        .expect("Height not set")
        .parse::<i32>()
        .expect("Error parsing height");

    let seed: String = match matches.value_of("seed") {
        Some(text) => {
            if text.chars().count() < 32 {
                panic!("Seed must be 32 characters long. Use -t option to create a new seed.")
            }
            text.to_string()
        }
        None => match matches.value_of("text") {
            Some(text) => create_hash(&text),
            None => create_hash(
                &thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .collect::<String>(),
            ),
        },
    };

    let walls = matches.is_present("walls");
    let method = match matches.value_of("algo").expect("Default algorithm not set") {
        "bsp" => Algorithm::Bsp,
        "rooms" => Algorithm::Rooms,
        _ => unreachable![],
    };

    let min_room_width: i32 = matches
        .value_of("minroomwidth")
        .expect("No room width")
        .parse()
        .expect("Error parsing min room width");
    let min_room_height: i32 = matches
        .value_of("minroomheight")
        .expect("No room height")
        .parse()
        .expect("Error parsing min room height");

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

    let level = match method {
        Algorithm::Rooms => RoomsCorridors::new(
            board_width,
            board_height,
            &seed,
            &mut rng,
            walls,
            min_room_width,
            min_room_height,
        ),
        Algorithm::Bsp => BspLevel::new(
            board_width,
            board_height,
            &seed,
            &mut rng,
            walls,
            min_room_width,
            min_room_height,
        ),
    };

    let print_json = matches.is_present("json");
    let draw_map = matches.is_present("draw");
    let csv = matches.is_present("csv");

    println!("{}", level);
    if print_json {
        let serialised = serde_json::to_string(&level).expect("Serialising level failed");
        println!("{}", serialised);
    }

    if draw_map {
        draw(&level, "./img", &seed).expect("Drawing failed");
    }

    if csv {
        println!("{:?}", level.board_to_csv());
    }
}

// include pre-generated rooms
// add detail to rooms
// drunkards walk
// bresenhams line algorithm
// non-rectangular rooms
// quadtree
// grid (gen on top + pick random direction)
// cellular automata https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664
// bsp https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268

// http://www.gamasutra.com/blogs/AAdonaac/20150903/252889/Procedural_Dungeon_Generation_Algorithm.php
