// use super::identifier::Identifier;
use crate::elements::identifier::Identifier;

#[allow(dead_code)]
struct Fraction {
    denominator: Identifier,
    numerator: Identifier
}

#[allow(dead_code)]
impl Fraction {
    fn new(numerator: &Identifier, denominator: &Identifier) -> Self {
        Self {
            numerator: numerator.clone(),
            denominator: denominator.clone(),
        }
    }

    fn get_numerator(&self) -> &Identifier {
        &self.numerator
    }

    fn get_denominator(&self) -> &Identifier {
        &self.denominator
    }

    fn set_numerator(&mut self, value: &Identifier){
        self.numerator = value.clone();
    }

    fn set_denominator(&mut self, value: &Identifier){
        self.denominator = value.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_a_fraction(){
        let numerator = Identifier::new(String::from("a"));
        let denominator = Identifier::new(String::from("b"));
        let fraction = Fraction::new(&numerator, &denominator);

        assert_eq!(numerator, *fraction.get_numerator());
        assert_eq!(denominator, *fraction.get_denominator());
    }

    #[test]
    fn test_it_can_change_numerator(){
        let identifier_a = Identifier::new(String::from("a"));
        let identifier_b = Identifier::new(String::from("b"));
        let identifier_c = Identifier::new(String::from("c"));

        let mut fraction = Fraction::new(&identifier_a, &identifier_b);

        assert_eq!(identifier_a, *fraction.get_numerator());

        fraction.set_numerator(&identifier_c);
        assert_eq!(identifier_c, *fraction.get_numerator());
    }

    #[test]
    fn test_it_can_change_denominator(){
        let identifier_a = Identifier::new(String::from("a"));
        let identifier_b = Identifier::new(String::from("b"));
        let identifier_c = Identifier::new(String::from("c"));

        let mut fraction = Fraction::new(&identifier_a, &identifier_b);

        assert_eq!(identifier_b, *fraction.get_denominator());

        fraction.set_denominator(&identifier_c);
        assert_eq!(identifier_c, *fraction.get_denominator());
    }
}
