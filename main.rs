use std::io;
use std::collections::HashSet;
use rand::seq::SliceRandom; 
use rand::thread_rng;       

fn print_hangman(wrong_guesses: usize) {
    let hangman_states = [
        r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
        r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
    ];
    println!("{}", hangman_states[wrong_guesses]);
}

fn main() {
    let word_bank = [
        "rustacean", "compiler", "ownership", "borrow", "cargo", "trait", "lifetime",
        "algorithm", "binary", "cache", "debug", "enum", "function", "generic", "hashmap",
        "iterator", "json", "keyword", "library", "macro", "namespace", "object", "pointer",
        "queue", "recursion", "stack", "thread", "unicode", "vector", "web", "xml",
        "array", "buffer", "closure", "data", "element", "feature", "guard", "heap",
        "index", "join", "key", "link", "module", "node", "option", "panic", "query",
        "result", "slice", "token", "unsafe", "value", "wrap", "yield", "zero", "async",
        "block", "const", "derive", "event", "field", "graph", "host", "input", "join",
        "kind", "lock", "match", "node", "open", "parse", "queue", "ref", "sync", "trait",
        "unwrap", "var", "while", "xor", "yield", "zip", "actor", "bind", "clone", "drop",
        "env", "flag", "gen", "hint", "impl", "join", "kind", "len", "map", "null", "ord",
        "path", "quit", "read", "send", "test", "unit", "view", "wait", "xor", "yield",
        "zone", "abort", "base", "call", "diff", "echo", "fork", "goto", "host", "init",
        "join", "kill", "loop", "main", "noop", "open", "ping", "quit", "read", "seek",
        "trap", "undo", "void", "warn", "exec", "file", "glob", "hash", "icon", "join",
        "kick", "list", "mute", "node", "open", "port", "quit", "root", "sync", "time",
        "user", "view", "wait", "xray", "yarn", "zone", "alpha", "bravo", "charlie",
        "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliet", "kilo", "lima",
        "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
        "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
        "apple", "banana", "cherry", "dragonfruit", "elderberry", "fig", "grape", "honeydew",
        "kiwi", "lemon", "mango", "nectarine", "orange", "papaya", "quince", "raspberry",
        "strawberry", "tangerine", "ugli", "vanilla", "watermelon", "xigua", "yam", "zucchini",
        "ant", "bee", "cat", "dog", "eel", "fox", "goat", "hawk", "ibis", "jay", "koala",
        "lion", "moose", "newt", "owl", "pig", "quail", "rat", "seal", "toad", "urchin",
        "vole", "wolf", "yak", "zebra", "arch", "bridge", "castle", "dock", "easel", "fence",
        "gate", "hut", "igloo", "jail", "kiln", "lodge", "mill", "nest", "obelisk", "pier",
        "quarry", "ranch", "shed", "tower", "urn", "villa", "well", "yurt", "zeppelin",
        "amber", "bronze", "copper", "diamond", "emerald", "feldspar", "garnet", "helium",
        "iron", "jade", "krypton", "lithium", "magnetite", "nickel", "onyx", "platinum",
        "quartz", "ruby", "silver", "topaz", "uranium", "vanadium", "wolfram", "xenon",
        "yellowcake", "zircon", "atlas", "bison", "canyon", "desert", "estuary", "fjord",
        "glacier", "hill", "island", "jungle", "knoll", "lagoon", "mesa", "narrows", "oasis",
        "plain", "quagmire", "ridge", "savanna", "tundra", "upland", "valley", "wetland",
        "xeric", "yardang", "zone", "actor", "baker", "carver", "diver", "editor", "farmer",
        "gardener", "hunter", "inventor", "jester", "knight", "lawyer", "miner", "nurse",
        "officer", "pilot", "queen", "ranger", "sailor", "teacher", "umpire", "veteran",
        "waiter", "xylophonist", "yodeler", "zoologist", "anchor", "beacon", "candle",
        "drum", "engine", "faucet", "gasket", "hinge", "igniter", "joist", "keystone",
        "latch", "motor", "nozzle", "outlet", "pulley", "quoin", "rivet", "spring", "toggle",
        "valve", "washer", "yoke", "zipper", "acorn", "bud", "cone", "daisy", "elm", "fern",
        "grass", "hazel", "ivy", "juniper", "kudzu", "lily", "moss", "nut", "oak", "petal",
        "quince", "reed", "spruce", "thorn", "umbel", "vine", "willow", "yucca", "zinnia",
        "aster", "birch", "cedar", "dogwood", "elder", "fir", "gum", "hemlock", "iris",
        "jasmine", "kale", "laurel", "maple", "nectar", "olive", "palm", "quassia", "rose",
        "sage", "tulip", "violet", "walnut", "yew", "zenith", "azure", "beige", "crimson",
        "denim", "ebony", "fuchsia", "gold", "hazel", "indigo", "jade", "khaki", "lavender",
        "magenta", "navy", "ochre", "peach", "quartz", "red", "scarlet", "teal", "umber",
        "violet", "white", "yellow", "zinc", "alpine", "bamboo", "coral", "delta", "echo",
        "fjord", "grove", "harbor", "isle", "jetty", "knob", "ledge", "marsh", "narwhal",
        "orchid", "pond", "quokka", "reef", "shoal", "thicket", "uplift", "vista", "warren",
        "xerox", "yacht", "zen", "apex", "bluff", "crest", "dune", "escarp", "fen", "glen",
        "heath", "inlet", "junct", "kraal", "ledge", "moraine", "notch", "outcrop", "peak",
        "quoin", "ravine", "spur", "tor", "upland", "vale", "wold", "xeric", "yard", "zawn"
    ];
    let secret_word = word_bank.choose(&mut thread_rng()).unwrap(); 
    let mut guessed_letters = HashSet::new();
    let max_tries = 6;
    let mut wrong_guesses = 0;

    println!("Welcome to Hangman! Guess the word.");

    loop {
        print_hangman(wrong_guesses);

        let display_word: String = secret_word.chars()
            .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
            .collect();
        println!("Word: {}", display_word);

        if !display_word.contains('_') {
            println!("You won! The word was '{}'.", secret_word);
            break;
        }

        println!("Guesses left: {}", max_tries - wrong_guesses);
        println!("Enter a letter:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next();

        if let Some(c) = guess {
            if guessed_letters.contains(&c) {
                println!("You already guessed '{}'. Try another.", c);
                continue;
            }
            guessed_letters.insert(c);

            if !secret_word.contains(c) {
                wrong_guesses += 1;
                println!("Wrong guess!");
            }

            if wrong_guesses >= max_tries {
                print_hangman(wrong_guesses);
                println!("Game over! The word was '{}'.", secret_word);
                break;
            }
        } else {
            println!("Please enter a valid letter.");
        }
    }
}
