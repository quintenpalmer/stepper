use std::{env, fs, num};

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
            _ => Err("<direction> must be one of 'up' or 'down'".to_string()),
        }
    }
}

fn closest_up(steppable_values: Vec<f32>, current: f32) -> f32 {
    for value in steppable_values.iter() {
        if current < *value {
            return value.clone();
        }
    }
    return steppable_values[steppable_values.len() - 1];
}

fn closest_down(steppable_values: Vec<f32>, current: f32) -> f32 {
    for value in steppable_values.iter().rev() {
        if current > *value {
            return value.clone();
        }
    }
    return steppable_values[0];
}

fn resolve_new_value(
    direction: Direction,
    current_value: f32,
    mut steppable_values: Vec<f32>,
) -> f32 {
    steppable_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    match direction {
        Direction::Bottom => steppable_values[0],
        Direction::Down => closest_down(steppable_values, current_value),
        Direction::Up => closest_up(steppable_values, current_value),
        Direction::Top => steppable_values[steppable_values.len() - 1],
    }
}

fn resolve_steppable_values_from_config(config_file_path: String) -> Result<Vec<f32>, String> {
    let file_contents = fs::read_to_string(config_file_path).map_err(|e| format!("{:?}", e))?;
    let steppable_values = file_contents
        .split("\n")
        .into_iter()
        .filter(|line| line.len() != 0)
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.parse::<f32>())
        .collect::<Result<Vec<f32>, num::ParseFloatError>>()
        .map_err(|e| format!("{:?}", e))?;
    return Ok(steppable_values);
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("Must supply <direction>, <current-value>, and <config-file-path>".to_string());
    }

    let direction = Direction::parse(&args[1])?;
    let current_value = args[2]
        .parse::<f32>()
        .map_err(|_| "<current-value> must be a floating point number".to_string())?;
    let config_file_path = args[3].clone();

    let steppable_values = resolve_steppable_values_from_config(config_file_path)?;

    let new_value = resolve_new_value(direction, current_value, steppable_values);

    println!("{}", new_value);

    return Ok(());
}
