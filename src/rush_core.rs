use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Hex(String);

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
impl From<Rgb> for Hex {
    fn from(color: Rgb) -> Self {
        let red = if format!("{:x}", color.0).len() == 1 {
            format!("0{:x}", color.0)
        } else {
            format!("{:x}", color.0)
        };
        let green = if format!("{:x}", color.1).len() == 1 {
            format!("0{:x}", color.1)
        } else {
            format!("{:x}", color.1)
        };
        let blue = if format!("{:x}", color.2).len() == 1 {
            format!("0{:x}", color.2)
        } else {
            format!("{:x}", color.2)
        };

        Hex(format!("#{}{}{}", red, green, blue))
    }
}

impl Hex {
    pub fn make_shades(self) -> [Hex; 10] {
        let color = Rgb::from(self);

        let shade_50 = color.lighter(0.95);
        let shade_100 = color.lighter(0.76);
        let shade_200 = color.lighter(0.57);
        let shade_300 = color.lighter(0.38);
        let shade_400 = color.lighter(0.19);
        let shade_500 = color.clone();
        let shade_600 = color.darker(0.19);
        let shade_700 = color.darker(0.38);
        let shade_800 = color.darker(0.57);
        let shade_900 = color.darker(0.76);

        let shades_rgb = [
            shade_50, shade_100, shade_200, shade_300, shade_400, shade_500, shade_600, shade_700,
            shade_800, shade_900,
        ];
        let shades_hex = shades_rgb.map(|c| Hex::from(c));
        shades_hex
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

#[derive(Clone, Copy)]
struct Rgb(u8, u8, u8);

impl Rgb {
    fn lighter(&self, degree: f32) -> Self {
        Rgb(
            (self.0 as f32 + ((255 - self.0) as f32 * degree)).round() as u8,
            (self.1 as f32 + ((255 - self.1) as f32 * degree)).round() as u8,
            (self.2 as f32 + ((255 - self.2) as f32 * degree)).round() as u8,
        )
    }
    fn darker(&self, degree: f32) -> Self {
        Rgb(
            (self.0 as f32 - (self.0 as f32 * degree)).round() as u8,
            (self.1 as f32 - (self.1 as f32 * degree)).round() as u8,
            (self.2 as f32 - (self.2 as f32 * degree)).round() as u8,
        )
    }
}

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
