use std::io::Write;
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
"apple","banana","orange","grape","melon","peach","pear","plum","mango","lemon",
"lime","cherry","berry","apricot","fig","date","kiwi","papaya","guava","coconut",
"pineapple","avocado","nectarine","tangerine","persimmon","quince","currant","raisin","cranberry","blueberry",
"strawberry","raspberry","blackberry","pomegranate","passionfruit","lychee","kumquat","cantaloupe","honeydew","watermelon",
"carrot","broccoli","spinach","lettuce","kale","cabbage","turnip","radish","parsnip","zucchini",
"squash","pumpkin","onion","garlic","shallot","leek","celery","cucumber","tomato","pepper",
"eggplant","potato","yam","beet","corn","pea","bean","lentil","chickpea","edamame",
"artichoke","asparagus","okra","rhubarb","ginger","turmeric","basil","oregano","thyme","parsley",
"rosemary","sage","cilantro","mint","dill","chive","bay","marjoram","tarragon","saffron",
"bread","butter","cheese","cream","yogurt","milk","honey","syrup","sugar","salt",
"peppercorn","vinegar","oil","mustard","ketchup","mayonnaise","relish","jam","jelly","marmalade",
"beef","pork","chicken","turkey","duck","lamb","veal","salmon","tuna","cod",
"herring","trout","sardine","shrimp","lobster","crab","oyster","clam","mussel","scallop",
"squid","octopus","eel","anchovy","mackerel","barracuda","snapper","tilapia","halibut","flounder",
"lion","tiger","cheetah","leopard","panther","jaguar","cougar","bobcat","lynx","ocelot",
"wolf","fox","coyote","jackal","hyena","bear","panda","koala","sloth","monkey",
"gorilla","chimp","baboon","lemur","gibbon","orangutan","elephant","rhino","hippo","buffalo",
"bison","moose","deer","elk","antelope","gazelle","giraffe","zebra","camel","llama",
"alpaca","goat","sheep","cow","horse","donkey","pig","rabbit","hare","squirrel",
"chipmunk","beaver","otter","weasel","ferret","badger","mole","bat","hedgehog","porcupine",
"platypus","kangaroo","wallaby","wombat","emu","ostrich","penguin","pelican","heron","stork",
"flamingo","parrot","macaw","cockatoo","toucan","sparrow","robin","swallow","swift","wren",
"finch","canary","lark","crow","raven","magpie","jay","starling","oriole","warbler",
"hummingbird","woodpecker","owl","eagle","hawk","falcon","kestrel","buzzard","vulture","condor",
"python","cobra","viper","rattlesnake","boa","anaconda","iguana","chameleon","gecko","monitor",
"crocodile","alligator","caiman","gavial","frog","toad","salamander","newt","turtle","tortoise",
"cricket","grasshopper","locust","mantis","beetle","ladybug","weevil","termite","ant","wasp",
"bee","hornet","dragonfly","damselfly","butterfly","moth","firefly","mosquito","flea","tick",
"spider","scorpion","centipede","millipede","snail","slug","clam","oyster","squid","octopus",
"cloud","rain","storm","lightning","thunder","snow","hail","sleet","fog","mist",
"breeze","wind","hurricane","tornado","cyclone","earthquake","volcano","tsunami","avalanche","flood",
"river","stream","creek","lake","pond","ocean","sea","bay","gulf","lagoon",
"reef","island","peninsula","cape","desert","dune","forest","jungle","swamp","marsh",
"meadow","plain","valley","hill","mountain","ridge","cliff","canyon","cave","grotto",
"rock","stone","pebble","boulder","sand","soil","mud","clay","dust","ash",
"gold","silver","copper","iron","lead","zinc","tin","nickel","platinum","diamond",
"ruby","sapphire","emerald","opal","pearl","amber","topaz","garnet","jade","quartz",
"chair","table","desk","sofa","couch","bed","cabinet","shelf","drawer","closet",
"lamp","mirror","clock","vase","rug","curtain","pillow","blanket","towel","mat",
"cup","mug","glass","plate","bowl","fork","knife","spoon","pan","pot",
"kettle","teapot","thermos","bottle","jug","bucket","basket","box","bag","case",
"radio","television","camera","laptop","tablet","phone","watch","headphone","speaker","microphone",
"keyboard","mouse","monitor","printer","scanner","router","modem","drone","robot","sensor",
"rocket","satellite","spaceship","planet","star","galaxy","comet","asteroid","meteor","nebula",
"city","village","town","capital","harbor","port","airport","station","bridge","tunnel",
"road","highway","street","avenue","alley","path","trail","track","lane","drive",
"school","college","library","museum","theater","cinema","stadium","arena","park","zoo",
"garden","farm","orchard","vineyard","barn","silo","mill","workshop","factory","warehouse",
"castle","palace","temple","church","mosque","synagogue","pagoda","monastery","fortress","tower",
"wall","gate","fence","arch","statue","fountain","monument","obelisk","pyramid","column",
"red","blue","green","yellow","orange","purple","pink","brown","black","white",
"gray","silver","golden","cyan","magenta","navy","teal","maroon","olive","beige",
"happy","sad","angry","brave","calm","eager","fierce","gentle","honest","kind",
"lazy","proud","shy","smart","strong","weak","young","old","rich","poor",
"fast","slow","big","small","tall","short","thick","thin","heavy","light",
"clean","dirty","dry","wet","warm","cold","soft","hard","sweet","sour",
"bitter","salty","fresh","stale","new","ancient","modern","simple","complex","clear",
"cloudy","bright","dark","noisy","quiet","empty","full","open","closed","early",
"late","near","far","high","low","deep","shallow","smooth","rough","sharp",
"round","square","flat","steep","wide","narrow","long","short","even","odd",
"sing","dance","run","walk","jump","swim","climb","crawl","fly","drive",
"write","read","draw","paint","build","break","catch","throw","push","pull",
"hold","drop","open","close","cut","burn","freeze","melt","grow","shrink",
"laugh","cry","smile","frown","sleep","wake","dream","think","know","learn",
"teach","play","watch","listen","talk","shout","whisper","search","find","hide",
"win","lose","begin","end","give","take","love","hate","hope","fear",
"plan","trust","wish","help","share","wait","rest","try","fix","join",
"travel","explore","discover","imagine","create","change","remember","forget","decide","choose",
];
    let secret_word = word_bank.choose(&mut thread_rng()).unwrap();
    let mut guessed_letters = HashSet::new();
    let max_tries = 6;
    let mut wrong_guesses = 0;

    println!("Welcome to Hangman! Guess the word.");

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();

        // Draw current hangman
        print_hangman(wrong_guesses);

        // Show word progress
        let display_word: String = secret_word.chars()
            .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
            .collect();
        println!("Word: {}", display_word);

        // Show used letters
        let mut used: Vec<char> = guessed_letters.iter().cloned().collect();
        used.sort();
        println!("Used letters: {:?}", used);

        // Check win
        if !display_word.contains('_') {
            println!("🎉 You won! The word was '{}'.", secret_word);
            break;
        }

        // Check lose
        if wrong_guesses >= max_tries {
            println!("😢 Game over! The word was '{}'.", secret_word);
            break;
        }

        println!("Guesses left: {}", max_tries - wrong_guesses);
        println!("Enter a letter:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next();

        if let Some(c) = guess {
            if !c.is_ascii_alphabetic() {
                println!("Please enter a letter.");
                continue;
            }

            let c = c.to_ascii_lowercase();

            if guessed_letters.contains(&c) {
                println!("You already guessed '{}'. Try another.", c);
                continue;
            }

            guessed_letters.insert(c);

            if !secret_word.contains(c) {
                wrong_guesses += 1;
            }
        } else {
            println!("Please enter a valid letter.");
        }
    }

    // Wait for Enter to exit
    println!("\nPress Enter to exit.");
    let mut exit = String::new();
    io::stdin().read_line(&mut exit).unwrap();
}
