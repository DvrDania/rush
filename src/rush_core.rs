use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Hex(pub String);

impl TryFrom<String> for Hex {
    // TODO: better error type
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let hex_color = Regex::new(r"^#(?:[0-9a-fA-F]{3}){1,2}$").unwrap();
        if let false = hex_color.is_match(&value) {
            return Err("Invalid color format".to_string());
        }
        Ok(Hex(value))
    }
}

impl Hex {
    pub fn make_shades(self) -> [Hex; 10] {
        let color = Rgb::from(self);
        [(); 10].map(|_| Hex("#ffffff".to_string()))
    }
    fn get_red(&self) -> u8 {
        let color: String = self.0.split('#').collect();
        if color.len() == 3 {
            let red = format!("{}{}", &color[0..1], &color[0..1]);
            let red = u8::from_str_radix(&red, 16).unwrap();
            red
        } else {
            let red = &color[0..2];
            let red = u8::from_str_radix(&red, 16).unwrap();
            red
        }
    }
    fn get_green(&self) -> u8 {
        let color: String = self.0.split('#').collect();
        if color.len() == 3 {
            let green = format!("{}{}", &color[1..2], &color[1..2]);
            let green = u8::from_str_radix(&green, 16).unwrap();
            green
        } else {
            let green = &color[2..4];
            let green = u8::from_str_radix(&green, 16).unwrap();
            green
        }
    }
    fn get_blue(&self) -> u8 {
        let color: String = self.0.split('#').collect();
        if color.len() == 3 {
            let blue = format!("{}{}", &color[2..3], &color[2..3]);
            let blue = u8::from_str_radix(&blue, 16).unwrap();
            blue
        } else {
            let blue = &color[4..6];
            let blue = u8::from_str_radix(&blue, 16).unwrap();
            blue
        }
    }
}

struct Rgb(u8, u8, u8);

impl From<Hex> for Rgb {
    fn from(color: Hex) -> Self {
        let red = color.get_red();
        let green = color.get_green();
        let blue = color.get_blue();
        Rgb(red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use crate::rush_core::Hex;

    #[test]
    fn valid_ffffff() {
        assert_eq!(
            Hex::try_from("#ffffff".to_string()),
            Ok(Hex("#ffffff".to_string()))
        );
    }
    #[test]
    fn valid_000000() {
        assert_eq!(
            Hex::try_from("#000000".to_string()),
            Ok(Hex("#000000".to_string()))
        );
    }
    #[test]
    fn valid_fff() {
        assert_eq!(
            Hex::try_from("#fff".to_string()),
            Ok(Hex("#fff".to_string()))
        );
    }
    #[test]
    fn invalid_a02f1g() {
        assert_eq!(
            Hex::try_from("#a02f1g".to_string()),
            Err("Invalid color format".to_string())
        );
    }
    #[test]
    fn invalid_fffff() {
        assert_eq!(
            Hex::try_from("#fffff".to_string()),
            Err("Invalid color format".to_string())
        );
    }
}
