#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    value: String
}

#[allow(dead_code)]
impl Identifier {
    pub fn new(value: String) -> Self {
        Self {
            value
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_an_identifier(){
        let identifier = Identifier::new(String::from("x"));

        assert_eq!(String::from("x"), identifier.get_value());
    }
}
