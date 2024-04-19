#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct Numeric {
    value: i32
}

#[allow(dead_code)]
impl Numeric {
    pub fn new(value: i32) -> Self {
        Self {
            value
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_a_numeric(){
        let numric = Numeric::new(2);

        assert_eq!(2, numric.get_value());
    }
}
