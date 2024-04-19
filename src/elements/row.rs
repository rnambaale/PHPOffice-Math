use crate::elements::identifier::Identifier;

#[allow(dead_code)]
struct Row {
    elements: Vec<Identifier>,
}

#[allow(dead_code)]
impl Row {
    fn new() -> Self {
        Self {
            elements: Vec::new()
        }
    }

    fn get_elements(&self) -> &Vec<Identifier> {
        &self.elements
    }

    fn add(&mut self, element: &Identifier) {
        self.elements.push(element.clone());
    }

    fn remove(&mut self, element: &Identifier) {
        match self.elements.iter().position(|e| e.get_value() == element.get_value()) {
            Some(index) => {
                self.elements.remove(index);
            },
            None => {

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_a_row(){
        let row = Row::new();
        let empty_vec: Vec<Identifier> = Vec::new();

        assert_eq!(&empty_vec, row.get_elements());
    }

    #[test]
    fn it_adds_element_to_row() {
        let mut row = Row::new();
        let empty_vec: Vec<Identifier> = Vec::new();

        assert_eq!(empty_vec, *row.get_elements());

        let element = Identifier::new(String::from("a"));
        row.add(&element);

        assert_eq!(vec![element], *row.get_elements());
    }

    #[test]
    fn it_removes_element_from_row(){
        let mut row = Row::new();
        let element = Identifier::new(String::from("a"));
        row.add(&element);
        assert_eq!(vec![element.clone()], *row.get_elements());

        row.remove(&element);

        let empty_vec: Vec<Identifier> = Vec::new();

        assert_eq!(empty_vec, *row.get_elements());
    }
}