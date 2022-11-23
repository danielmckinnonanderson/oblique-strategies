use rand::Rng;

pub fn run() {
    let random_index = get_random_index(&STRATEGIES.len());

    let strategy = get_random_strategy_text(STRATEGIES, random_index);

    match strategy {
        Ok(s) => println!("\n{}\n", s),
        Err(_) => println!("Error printing strategy"),
    }
}

// TODO - fancy print function, print as cards

fn get_random_strategy_text<'a>(
    strategies: &'static [ObliqueStrategy],
    index: usize,
) -> Result<&'static str, &'a str> {
    match strategies.get(index) {
        Some(o) => Ok(o.text),
        None => Err("Could not access element"),
    }
}

fn get_random_index(_len: &usize) -> usize {
    // TODO - interpret length in range
    rand::thread_rng().gen_range(1..=132) - 1
}

struct ObliqueStrategy<'a> {
    pub text: &'a str,
}

const STRATEGIES: &'static [ObliqueStrategy] = &[
    ObliqueStrategy {
        text: "Define an area as 'safe' and use it as an anchor.",
    },
    ObliqueStrategy {
        text: "A line has two sides",
    },
    ObliqueStrategy {
        text: "The most important thing is the thing most forgotten.",
    },
    ObliqueStrategy {
        text: "The inconsistency principle.",
    },
    ObliqueStrategy {
        text: "Not building a wall but making a brick",
    },
    ObliqueStrategy {
        text: "Remember those quiet evenings",
    },
    ObliqueStrategy {
        text: "Don't break the silence",
    },
    ObliqueStrategy {
        text: "Ask your body",
    },
    ObliqueStrategy {
        text: "Work at a different speed",
    },
    ObliqueStrategy {
        text: "You can only make one dot at a time",
    },
    ObliqueStrategy {
        text: "Make a blank valuable by putting it in an exquisite frame",
    },
    ObliqueStrategy {
        text: "Short circuit (If eating peas improves virility, shovel them into your pants)",
    },
    ObliqueStrategy {
        text: "Breathe more deeply",
    },
    ObliqueStrategy {
        text: "Balance the consistency principle with the inconsistency principle",
    },
    ObliqueStrategy {
        text: "Is the intonation correct",
    },
    ObliqueStrategy {
        text: "Cluster analysis",
    },
    ObliqueStrategy {
        text: "Bridges -build -burn",
    },
    ObliqueStrategy {
        text: "Infinitesimal gradations",
    },
    ObliqueStrategy {
        text: "You don't have to be ashamed of using your own ideas",
    },
    ObliqueStrategy {
        text: "Lowest common denominator",
    },
    ObliqueStrategy {
        text: "Use an old idea",
    },
    ObliqueStrategy {
        text: "Get your neck massaged",
    },
    ObliqueStrategy {
        text: "Trust in the you of now",
    },
    ObliqueStrategy {
        text: "Honor thy error as a hidden intention",
    },
    ObliqueStrategy {
        text: "Retrace your steps",
    },
    ObliqueStrategy {
        text: "Once the search is in progress, something will be found",
    },
    ObliqueStrategy {
        text: "Left channel, right channel, centre channel",
    },
    ObliqueStrategy {
        text: "Revaluation (a warm feeling)",
    },
    ObliqueStrategy {
        text: "What is the reality of the situation",
    },
    ObliqueStrategy {
        text: "From nothing to more than nothing",
    },
    ObliqueStrategy {
        text: "Are there sections? Consider transitions",
    },
    ObliqueStrategy {
        text: "Just carry on",
    },
    ObliqueStrategy {
        text: "Use 'unqualified' people",
    },
    ObliqueStrategy {
        text: "Do the words need changing?",
    },
    ObliqueStrategy {
        text: "Use filters",
    },
    ObliqueStrategy {
        text: "Is there something missing?",
    },
    ObliqueStrategy {
        text: "Don't be afraid of things because they're easy to do",
    },
    ObliqueStrategy { text: "Tidy up" },
    ObliqueStrategy {
        text: "Convert a melodic element to a rhythmic element",
    },
    ObliqueStrategy {
        text: "Put in earplugs",
    },
    ObliqueStrategy {
        text: "Go slowly all the way round the outside",
    },
    ObliqueStrategy {
        text: "Be less critical more often",
    },
    ObliqueStrategy {
        text: "Think of the radio",
    },
    ObliqueStrategy {
        text: "Emphasize repetitions",
    },
    ObliqueStrategy { text: "Water" },
    ObliqueStrategy { text: "Cascades" },
    ObliqueStrategy {
        text: "Don't be frightened of cliches",
    },
    ObliqueStrategy {
        text: "Make a sudden, destructive unpredictable action; incorporate",
    },
    ObliqueStrategy { text: "Courage!" },
    ObliqueStrategy {
        text: "Retrace your steps",
    },
    ObliqueStrategy {
        text: "Accept advice",
    },
    ObliqueStrategy {
        text: "Take a break",
    },
    ObliqueStrategy {
        text: "Disconnect from desire",
    },
    ObliqueStrategy {
        text: "Only a part, not the whole",
    },
    ObliqueStrategy {
        text: "Consider diferent fading systems",
    },
    ObliqueStrategy {
        text: "Humanize something free of error",
    },
    ObliqueStrategy {
        text: "Intentions -nobility of -humility of -credibility of",
    },
    ObliqueStrategy {
        text: "Into the impossible",
    },
    ObliqueStrategy {
        text: "You are engineer",
    },
    ObliqueStrategy {
        text: "Change nothing and continue with immaculate consistency",
    },
    ObliqueStrategy {
        text: "Abandon normal instruments",
    },
    ObliqueStrategy {
        text: "Discard an axiom",
    },
    ObliqueStrategy {
        text: "What are you really thinking about just now",
    },
    ObliqueStrategy {
        text: "Listen to the quiet voice",
    },
    ObliqueStrategy {
        text: "Remove specifics and convert to ambiguities",
    },
    ObliqueStrategy {
        text: "Go to an extreme, move back to a comfortable place",
    },
    ObliqueStrategy {
        text: "What are the sections sections of? Imagine a caterpillar moving",
    },
    ObliqueStrategy {
        text: "Give way to your worst impulse",
    },
    ObliqueStrategy {
        text: "Destroy -nothing -the most important thing",
    },
    ObliqueStrategy { text: "   " },
    ObliqueStrategy {
        text: "  [blank]  ",
    },
    ObliqueStrategy {
        text: "State the problem in words as clearly as possible",
    },
    ObliqueStrategy {
        text: "Change instrument roles",
    },
    ObliqueStrategy {
        text: "Fill every beat with something",
    },
    ObliqueStrategy {
        text: "Twist the spine",
    },
    ObliqueStrategy {
        text: "Disciplined self-indulgence",
    },
    ObliqueStrategy {
        text: "Children -speaking -singing",
    },
    ObliqueStrategy {
        text: "Is it finished?",
    },
    ObliqueStrategy {
        text: "Decorate, decorate",
    },
    ObliqueStrategy {
        text: "Faced with a choice, do both",
    },
    ObliqueStrategy {
        text: "Question the heroic approach",
    },
    ObliqueStrategy {
        text: "What wouldn't you do?",
    },
    ObliqueStrategy {
        text: "Mute and continue",
    },
    ObliqueStrategy {
        text: "The tape is now the music",
    },
    ObliqueStrategy {
        text: "Cut a vital connection",
    },
    ObliqueStrategy {
        text: "Mechanize something idiosyncratic",
    },
    ObliqueStrategy {
        text: "Would anybody want it?",
    },
    ObliqueStrategy {
        text: "Emphasize differences",
    },
    ObliqueStrategy {
        text: "Simply a matter of work",
    },
    ObliqueStrategy {
        text: "Always give yourself credit for having more than personality",
    },
    ObliqueStrategy {
        text: "Do the washing up",
    },
    ObliqueStrategy {
        text: "Do we need holes",
    },
    ObliqueStrategy {
        text: "Take away the elements in order of apparent non-importance",
    },
    ObliqueStrategy {
        text: "Ghost echoes",
    },
    ObliqueStrategy {
        text: "Look closely at the most embarrassing details and amplify them",
    },
    ObliqueStrategy {
        text: "What mistakes did you make last time?",
    },
    ObliqueStrategy { text: "Reverse" },
    ObliqueStrategy {
        text: "Use an unacceptable color",
    },
    ObliqueStrategy {
        text: "(Organic) machinery",
    },
    ObliqueStrategy {
        text: "Emphasize the flaws",
    },
    ObliqueStrategy {
        text: "It is quite possible (after all)",
    },
    ObliqueStrategy {
        text: "Give the game away",
    },
    ObliqueStrategy {
        text: "Be extravagant",
    },
    ObliqueStrategy {
        text: "Idiot glee (?)",
    },
    ObliqueStrategy { text: "Accretion" },
    ObliqueStrategy {
        text: "Tape your mouth",
    },
    ObliqueStrategy {
        text: "Remove ambiguities and convert to specifics",
    },
    ObliqueStrategy {
        text: "Imagine the piece as a set of disconnected events",
    },
    ObliqueStrategy {
        text: "How would you have done it?",
    },
    ObliqueStrategy {
        text: "Spectrum analysis",
    },
    ObliqueStrategy {
        text: "Feed the recording back out of the medium",
    },
    ObliqueStrategy {
        text: "Only one element of each kind",
    },
    ObliqueStrategy {
        text: "Look at the order in which you do things",
    },
    ObliqueStrategy {
        text: "In total darkness, or in a very large room, very quietly",
    },
    ObliqueStrategy {
        text: "Repetition is a form of change",
    },
    ObliqueStrategy {
        text: "Overtly resist change",
    },
    ObliqueStrategy {
        text: "Assemble some of the elements in a group and treat the group",
    },
    ObliqueStrategy {
        text: "Don't stress one thing more than another",
    },
    ObliqueStrategy {
        text: "Do something boring",
    },
    ObliqueStrategy {
        text: "Distorting time",
    },
    ObliqueStrategy {
        text: "Go outside. Shut the door.",
    },
    ObliqueStrategy {
        text: "Discover the recipes you are using and abandon them",
    },
    ObliqueStrategy {
        text: "What would your closest friend do?",
    },
    ObliqueStrategy {
        text: "Use fewer notes",
    },
    ObliqueStrategy {
        text: "Consult other sources -promising -unpromising",
    },
    ObliqueStrategy {
        text: "Do nothing for as long as possible",
    },
    ObliqueStrategy {
        text: "Turn it upside down",
    },
    ObliqueStrategy {
        text: "Allow an easement (an easement is the abandonment of a stricture)",
    },
    ObliqueStrategy {
        text: "Simple subtraction",
    },
    ObliqueStrategy {
        text:
            "Make an exhaustive list of everything you might do and do the last thing on the list",
    },
    ObliqueStrategy {
        text: "Ask people to work against their better judgement",
    },
    ObliqueStrategy {
        text: "Don't be frightened to display your talents",
    },
];
