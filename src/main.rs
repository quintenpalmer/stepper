use std::{cmp, env, fmt, fs, str};

pub enum Direction {
    Bottom,
    Down,
    Up,
    Top,
}

impl Direction {
    pub fn parse(s: &String) -> Result<Direction, String> {
        match s.as_str() {
            "bottom" => Ok(Direction::Bottom),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "top" => Ok(Direction::Top),
            _ => Err("<direction> must be one of 'top', 'up', 'down', or 'bottom'".to_string()),
        }
    }
}

fn closest_up<T: cmp::PartialOrd + Clone>(steppable_values: Vec<T>, current: T) -> T {
    for value in steppable_values.iter() {
        if current < *value {
            return value.clone();
        }
    }
    return steppable_values[steppable_values.len() - 1].clone();
}

fn closest_down<T: cmp::PartialOrd + Clone>(steppable_values: Vec<T>, current: T) -> T {
    for value in steppable_values.iter().rev() {
        if current > *value {
            return value.clone();
        }
    }
    return steppable_values[0].clone();
}

fn resolve_new_value<T: cmp::PartialOrd + Clone>(
    direction: Direction,
    current_value: T,
    mut steppable_values: Vec<T>,
) -> T {
    steppable_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match direction {
        Direction::Bottom => steppable_values[0].clone(),
        Direction::Down => closest_down(steppable_values, current_value),
        Direction::Up => closest_up(steppable_values, current_value),
        Direction::Top => steppable_values[steppable_values.len() - 1].clone(),
    }
}

fn resolve_steppable_values_from_config<T: str::FromStr>(
    config_file_path: String,
) -> Result<Vec<T>, String>
where
    <T as str::FromStr>::Err: fmt::Debug,
{
    let file_contents = fs::read_to_string(config_file_path).map_err(|e| format!("{:?}", e))?;
    let steppable_values = file_contents
        .split("\n")
        .into_iter()
        .filter(|line| line.len() != 0)
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|e| format!("{:?}", e))?;
    return Ok(steppable_values);
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        return Err(
            "Usage stepper <value-type> <direction> <current-value> <config-file-path>".to_string(),
        );
    }

    let value_type = args[1].clone();
    let direction = Direction::parse(&args[2])?;
    let config_file_path = args[4].clone();

    match value_type.as_str() {
        "f32" => {
            let current_value = args[3].parse::<f32>().map_err(|_| {
                "<value-type> of f32 means <current-value> must be a floating point number"
                    .to_string()
            })?;

            let steppable_values = resolve_steppable_values_from_config::<f32>(config_file_path)?;

            let new_value = resolve_new_value(direction, current_value, steppable_values);

            println!("{}", new_value);
        }
        "u32" => {
            let current_value = args[3].parse::<u32>().map_err(|_| {
                "<value-type> of u32 means <current-value> must be a positive integer".to_string()
            })?;

            let steppable_values = resolve_steppable_values_from_config::<u32>(config_file_path)?;

            let new_value = resolve_new_value(direction, current_value, steppable_values);

            println!("{}", new_value);
        }
        _ => return Err("value type must be one of `f32` or `u32`".to_string()),
    };

    return Ok(());
}
