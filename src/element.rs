#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    value: String
}

#[allow(dead_code)]
impl Element {
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
    fn it_constructs_an_element(){
        let element = Element::new(String::from("x"));

        assert_eq!(String::from("x"), element.get_value());
    }
}
