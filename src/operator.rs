#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Operator {
    value: String
}

#[allow(dead_code)]
impl Operator {
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
    fn it_constructs_an_operator(){
        let operator = Operator::new(String::from("+"));

        assert_eq!("+", operator.get_value());
    }
}
