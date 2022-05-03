use regex::Regex;



#[derive(Debug)]
pub struct Coordinate{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate{
    pub fn get_distance_between(&self, other: &Coordinate) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        (x*x + y*y + z*z).sqrt()
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}


#[derive(Debug, Clone)]
pub struct ParsingError;

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing error")
    }
}


pub fn is_valid(input: &str) -> bool {
    let regex = Regex::new(r"Coordinates: x:[\d-]+.\d+ y:[\d-]+.\d+ z:[\d-]+.\d+").unwrap();
    regex.is_match(input)
}

pub fn parse_coordinate(input: &str) -> Result<Coordinate, ParsingError>{
    let mut coordinate = Coordinate{
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let regex = Regex::new(r"[\d-]+.\d+").unwrap();
    let caps = regex.captures_iter(input);
    let mut i = 0;
    for cap in caps{
        let value = cap.get(0).unwrap().as_str().parse::<f64>().unwrap();
        match i {
            0 => {
                coordinate.x = value;
            },
            1 => {
                coordinate.y = value;
            },
            2 => {
                coordinate.z = value;
            },
            _ => {
                return Err(ParsingError);
            }
        }
        i+=1;
    }
    Ok(coordinate)
}


#[cfg(test)]
mod tests{
    use test_case::test_case;
    use super::*;

    #[test_case("Coordinates: x: y:0.0 z:-0.0", false)]
    #[test_case("Coordinates: x:0.0 y:0.0 z:0.0", true)]
    #[test_case("Coordinates: x:12792414426.801407 y:-74275565.552555 z:83180.938669", true)]
    fn test_is_valid(input: &str, expected: bool) {
        assert_eq!(is_valid(input), expected);
    }

    #[test]
    fn test_is_coordinate() {
        let input = "Coordinates: x:0.0 y:0.0 z:0.0";
        assert_eq!(parse_coordinate(input).unwrap(), Coordinate{x: 0.0, y: 0.0, z: 0.0});
    }

    #[test]
    fn test_is_long_coordinate() {
        let input = "Coordinates: x:12792414426.801407 y:-74275565.552555 z:83180.938669";
        assert_eq!(parse_coordinate(input).unwrap(), Coordinate{x: 12792414426.801407 , y: -74275565.552555 , z: 83180.938669});
    }
}