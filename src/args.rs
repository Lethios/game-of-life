pub struct Args {
    pub seed: u64,
    pub rulestring: ([bool; 9], [bool; 9]),
    pub speed: f64,
    pub spawn: f64,
}

pub fn parse_args() -> Result<Args, String> {
    let mut args = std::env::args().skip(1);
    let mut parsed_args = Args {
        // Set default arguments
        seed: rand::random::<u64>(),
        rulestring: (
            [false, false, false, true, false, false, false, false, false],
            [false, false, true, true, false, false, false, false, false],
        ),
        speed: 10.0,
        spawn: 0.5,
    };

    while let Some(flag) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("'{flag}' requires a value"))?;

        match flag.as_str() {
            "--seed" => parsed_args.seed = validate_seed(&value)?,
            "--rulestring" => parsed_args.rulestring = validate_rulestring(&value)?,
            "--speed" => parsed_args.speed = validate_speed(&value)?,
            "--spawn" => parsed_args.spawn = validate_spawn(&value)?,
            _ => return Err(format!("Invalid flag: {value}")),
        }
    }

    Ok(parsed_args)
}

fn validate_seed(seed: &str) -> Result<u64, String> {
    seed.parse::<u64>()
        .map_err(|_| format!("Invalid seed: {seed} is not a valid u64"))
}

fn parse_rulestring(rulestring: &str) -> Result<([bool; 9], [bool; 9]), String> {
    let mut birth_rules = [false; 9];
    let mut survival_rules = [false; 9];

    let parts: Vec<&str> = rulestring.split("/").collect();

    let birth = parts
        .get(0)
        .ok_or_else(|| format!("Invalid rulestring: expected format B<digits>/S<digits>"))?
        .strip_prefix("B")
        .ok_or_else(|| format!("Invalid rulestring: birth rule must start with 'B'"))?;
    let survival = parts
        .get(1)
        .ok_or_else(|| format!("Invalid rulestring: expected format B<digits>/S<digits>"))?
        .strip_prefix("S")
        .ok_or_else(|| format!("Invalid rulestring: survival rule must start with 'S'"))?;

    for b in birth.chars() {
        let num = b
            .to_digit(10)
            .ok_or_else(|| format!("Invalid character '{b}' in birth rule"))?;
        if num > 8 {
            return Err(format!("Invalid birth digit: {num} cannot exceed 8"));
        }

        birth_rules[num as usize] = true;
    }

    for s in survival.chars() {
        let num = s
            .to_digit(10)
            .ok_or_else(|| format!("Invalid character '{s}' in survival rule"))?;
        if num > 8 {
            return Err(format!("Invalid survival digit: {num} cannot exceed 8"));
        }

        survival_rules[num as usize] = true;
    }

    Ok((birth_rules, survival_rules))
}

fn validate_rulestring(rulestring: &str) -> Result<([bool; 9], [bool; 9]), String> {
    parse_rulestring(rulestring)
}

fn validate_speed(speed: &str) -> Result<f64, String> {
    speed
        .parse::<f64>()
        .map_err(|_| format!("Invalid speed: {speed} is not a valid f64"))
}

fn validate_spawn(spawn: &str) -> Result<f64, String> {
    let spawn = spawn
        .parse::<f64>()
        .map_err(|_| format!("Invalid spawn probability: {spawn} is not a valid f64"))?;

    if !(0.0..=1.0).contains(&spawn) {
        return Err(format!(
            "Invalid spawn probability: {spawn} is not in the range [0, 1]"
        ));
    }

    Ok(spawn)
}
