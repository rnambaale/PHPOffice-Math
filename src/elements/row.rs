use super::CommonElement;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Row {
    elements: Vec<CommonElement>,
}

#[allow(dead_code)]
impl Row {
    pub fn new() -> Self {
        Self {
            elements: Vec::new()
        }
    }

    pub fn get_elements(&self) -> &Vec<CommonElement> {
        &self.elements
    }

    pub fn add(&mut self, element: CommonElement){
        self.elements.push(element);
    }

    fn remove(&mut self, element: &CommonElement) {
        let index = self.elements.iter().position(|e| e == element);
        if let Some(index) = index {
            self.elements.remove(index);
        }
        // match self.elements.iter().position(|e| e.get_value() == element.get_value()) {
        //     Some(index) => {
        //         self.elements.remove(index);
        //     },
        //     None => {

        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use crate::elements::identifier::Identifier;

    use super::*;

    #[test]
    fn it_constructs_a_row(){
        let row = Row::new();
        let empty_vec: Vec<CommonElement> = Vec::new();

        assert_eq!(&empty_vec, row.get_elements());
    }

    #[test]
    fn it_adds_element_to_row() {
        let mut row = Row::new();
        let empty_vec: Vec<CommonElement> = Vec::new();

        assert_eq!(empty_vec, *row.get_elements());

        let element = Identifier::new(String::from("a"));
        row.add(CommonElement::Identifier(element.clone()));

        assert_eq!(vec![CommonElement::Identifier(element)], *row.get_elements());
    }

    #[test]
    fn it_removes_element_from_row(){
        let mut row = Row::new();
        let element = Identifier::new(String::from("a"));
        row.add(CommonElement::Identifier(element.clone()));
        assert_eq!(vec![CommonElement::Identifier(element.clone())], *row.get_elements());

        row.remove(&CommonElement::Identifier(element));

        let empty_vec: Vec<CommonElement> = Vec::new();

        assert_eq!(empty_vec, *row.get_elements());
    }
}
