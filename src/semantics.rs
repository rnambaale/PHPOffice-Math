use std::collections::HashMap;

#[allow(dead_code)]
pub struct Semantics {
    annotations: HashMap<String, String>
}

#[allow(dead_code)]
impl Semantics {
    pub fn new() -> Self {
        Self {
            annotations: HashMap::new()
        }
    }

    pub fn add_annotation(&mut self, encoding: &str, annotation: &str) {
        self.annotations.insert(encoding.to_string(), annotation.to_string());
    }

    pub fn get_annotations(&self) -> &HashMap<String, String>{
        &self.annotations
    }

    pub fn get_annotation(&self, encoding: &str) -> Option<&String> {
        self.annotations.get(encoding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_semantics(){
        let semantics = Semantics::new();
        let empty: HashMap<String, String> = HashMap::new();

        assert_eq!(&empty, semantics.get_annotations());
        assert_eq!(semantics.get_annotations().len(), 0);
    }

    #[test]
    fn it_adds_annotations(){
        let mut semantics = Semantics::new();
        let empty: HashMap<String, String> = HashMap::new();
        assert_eq!(&empty, semantics.get_annotations());
        assert_eq!(semantics.get_annotations().len(), 0);

        let encoding_str = String::from("encoding");
        let annotation_str = String::from("annotation");
        semantics.add_annotation(&encoding_str, &annotation_str);

        assert_eq!(Some(&annotation_str), semantics.get_annotations().get(&encoding_str));
        assert_eq!(semantics.get_annotations().len(), 1);

        assert_eq!(Some(&annotation_str), semantics.get_annotation(&encoding_str));
        assert_eq!(None, semantics.get_annotation(String::from("notexisting").as_str()));
    }
}
