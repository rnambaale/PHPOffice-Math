use crate::element::Element;

#[allow(dead_code)]
pub struct Superscript {
    base: Element,
    superscript: Element,
}

#[allow(dead_code)]
impl Superscript {
    pub fn new(base: &Element, superscript: &Element) -> Self {
        Self {
            base: base.clone(),
            superscript: superscript.clone()
        }
    }

    pub fn get_base(&self) -> &Element {
        &self.base
    }

    pub fn set_base(&mut self, value: &Element) {
        self.base = value.clone();
    }

    pub fn get_superscript(&self) -> &Element {
        &self.superscript
    }

    pub fn set_superscript(&mut self, value: &Element) {
        self.superscript = value.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_a_superscript(){
        let identifier_a = Element::new(String::from("a"));
        let identifier_b = Element::new(String::from("b"));
        let superscript = Superscript::new(&identifier_a, &identifier_b);

        assert_eq!(&identifier_a, superscript.get_base());
        assert_eq!(&identifier_b, superscript.get_superscript());
    }

    #[test]
    fn it_can_update_the_base() {
        let identifier_a = Element::new(String::from("a"));
        let identifier_b = Element::new(String::from("b"));
        let identifier_c = Element::new(String::from("c"));

        let mut superscript = Superscript::new(&identifier_a, &identifier_b);

        assert_eq!(&identifier_a, superscript.get_base());
        superscript.set_base(&identifier_c);
        assert_eq!(&identifier_c, superscript.get_base());
    }

    #[test]
    fn it_can_update_the_superscript_value() {
        let identifier_a = Element::new(String::from("a"));
        let identifier_b = Element::new(String::from("b"));
        let identifier_c = Element::new(String::from("c"));

        let mut superscript = Superscript::new(&identifier_a, &identifier_b);

        assert_eq!(&identifier_b, superscript.get_superscript());
        superscript.set_superscript(&identifier_c);
        assert_eq!(&identifier_c, superscript.get_superscript());
    }
}
