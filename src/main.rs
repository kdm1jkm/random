use std::fmt::Display;

use inquire::{Select, Text};
use random::Generator;

enum GeneratorType {
    Lgc,
    Msm,
    XorShift,
}

impl Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneratorType::Lgc => write!(f, "Linear Congruential Generator (LGC)"),
            GeneratorType::Msm => write!(f, "Mid-Square Method (MSM)"),
            GeneratorType::XorShift => write!(f, "XorShift"),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let selection = Select::new(
        "Choose generator",
        vec![
            GeneratorType::Lgc,
            GeneratorType::Msm,
            GeneratorType::XorShift,
        ],
    )
    .prompt()?;

    let seed: u32 = Text::new("Enter seed").prompt()?.parse()?;

    let loop_count: usize = Text::new("Enter number of times").prompt()?.parse()?;

    let mut generator = match selection {
        GeneratorType::Lgc => Box::new(random::LGC::new(seed)) as Box<dyn Generator>,
        GeneratorType::Msm => Box::new(random::MSM::new(seed)) as Box<dyn Generator>,
        GeneratorType::XorShift => Box::new(random::XorShift::new(seed)) as Box<dyn Generator>,
    };

    for _ in 0..loop_count {
        println!("{}", generator.next_value());
    }
    Ok(())
}
