use std::{cmp, env, fmt, fs, path, str};

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

pub struct Stepper<T> {
    values: Vec<T>,
}

impl<T> Stepper<T>
where
    T: str::FromStr + Clone + cmp::PartialOrd,
    <T as str::FromStr>::Err: fmt::Debug,
{
    fn closest_up(&self, current: T) -> T {
        for value in self.values.iter() {
            if current < *value {
                return value.clone();
            }
        }
        return self.values[self.values.len() - 1].clone();
    }

    fn closest_down(&self, current: T) -> T {
        for value in self.values.iter().rev() {
            if current > *value {
                return value.clone();
            }
        }
        return self.values[0].clone();
    }

    pub fn resolve_new_value(&self, direction: Direction, current_value: T) -> T {
        match direction {
            Direction::Bottom => self.values[0].clone(),
            Direction::Down => self.closest_down(current_value),
            Direction::Up => self.closest_up(current_value),
            Direction::Top => self.values[self.values.len() - 1].clone(),
        }
    }

    pub fn from_file<P: AsRef<path::Path>>(filename: P) -> Result<Self, String> {
        let file_contents = fs::read_to_string(filename).map_err(|e| format!("{:?}", e))?;
        let mut steppable_values = file_contents
            .split("\n")
            .into_iter()
            .filter(|line| line.len() != 0)
            .filter(|line| !line.starts_with('#'))
            .map(|line| line.parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|e| format!("{:?}", e))?;

        steppable_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        return Ok(Stepper {
            values: steppable_values,
        });
    }
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

            let stepper: Stepper<f32> = Stepper::from_file(config_file_path)?;

            let new_value = stepper.resolve_new_value(direction, current_value);

            println!("{}", new_value);
        }
        "u32" => {
            let current_value = args[3].parse::<u32>().map_err(|_| {
                "<value-type> of u32 means <current-value> must be a positive integer".to_string()
            })?;

            let stepper: Stepper<u32> = Stepper::from_file(config_file_path)?;

            let new_value = stepper.resolve_new_value(direction, current_value);

            println!("{}", new_value);
        }
        _ => return Err("value type must be one of `f32` or `u32`".to_string()),
    };

    return Ok(());
}
