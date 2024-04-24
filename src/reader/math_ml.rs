
use crate::elements::operator::Operator;
use crate::elements::superscript::Superscript;
use crate::elements::CommonElement;
use crate::{elements::identifier::Identifier, reader::reader_interface::ReaderInterface};
use crate::elements::row::Row;
use xml::reader::{EventReader, XmlEvent};

#[allow(dead_code)]
pub struct MathMl {
    math: Vec<Row>,
}

#[derive(Debug)]
enum Common {
    Row(Row),
    Identifier(Identifier),
    Operator(Operator),
    Superscript(Superscript),
}

#[allow(dead_code)]
impl ReaderInterface for MathMl {
    fn read(&mut self, content: &str) -> Result<(), String> {
        let content = content.replace("&InvisibleTimes;", "InvisibleTimes");

        let mut elements_stack: Vec<Common> = Vec::new();

        let parser = EventReader::from_str(&content);

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    let element_name = name.local_name;

                    match element_name.as_str() {
                        "mrow" => elements_stack.push(Common::Row(Row::new())),
                        "mi" => elements_stack.push(Common::Identifier(Identifier::new(String::from("")))),
                        "mo" => elements_stack.push(Common::Operator(Operator::new(String::from("")))),
                        "msup" => elements_stack.push(
                            Common::Superscript(
                                Superscript::new(&Identifier::new(String::from("")), &Identifier::new(String::from("")))
                            )
                        ),
                        "mn" => elements_stack.push(Common::Identifier(Identifier::new(String::from("")))),
                        _ => {}
                    }

                }

                Ok(XmlEvent::Characters(text)) => {
                    match elements_stack.last_mut() {
                        Some(Common::Identifier(element)) => {
                            element.set_value(&text);
                        },
                        Some(Common::Operator(element)) => {
                            element.set_value(&text);
                        }
                        None => {},
                        _ => {}
                    }
                }

                Ok(XmlEvent::EndElement { name }) => {
                    let element_name = name.local_name;
                    match element_name.as_str() {
                        "mi" => {
                            if let Some(Common::Identifier(element)) = elements_stack.pop() {
                                let parent = elements_stack.last_mut().unwrap();
                                if let Common::Row(row) = parent {
                                    row.add(CommonElement::Identifier(element.clone()));
                                }

                                if let Common::Superscript(superscript) = parent {
                                    superscript.set_base(&element);
                                }
                            }
                        },
                        "mo" => {
                            if let Some(Common::Operator(element)) = elements_stack.pop() {
                                let parent = elements_stack.last_mut().unwrap();
                                if let Common::Row(row) = parent {
                                    row.add(CommonElement::Operator(element.clone()));
                                }
                            }
                        }
                        "msup" => {
                            if let Some(Common::Identifier(superscript_identifier)) = elements_stack.pop() {
                                let parent = elements_stack.last_mut().unwrap();

                                if let Common::Superscript(superscript) = parent {
                                    superscript.set_superscript(&superscript_identifier);
                                }
                            }

                            if let Some(Common::Superscript(superscript)) = elements_stack.pop() {
                                let parent = elements_stack.last_mut().unwrap();
                                if let Common::Row(row) = parent {
                                    row.add(CommonElement::Superscript(superscript.clone()));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Err(e) => {
                    dbg!("{}", &e);
                    return Err(format!("Error: {}", e));
                },
                _ => {}
            }
        }
        let mut math: Vec<Row> = Vec::new();
        for item in &elements_stack {
            match item {
                Common::Row(row) => math.push(row.clone()),
                _ => {}
            }
        }
        self.math = math;

        Ok(())
    }
}

#[allow(dead_code)]
impl MathMl {
    fn new() -> Self {
        Self {
            math: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_performs_basic_read(){
        let content = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <!DOCTYPE math PUBLIC \"-//W3C//DTD MathML 2.0//EN\" \"http://www.w3.org/Math/DTD/mathml2/mathml2.dtd\">
        <math xmlns=\"http://www.w3.org/1998/Math/MathML\">
            <mrow>
                <mi>a</mi> <mo>&InvisibleTimes;</mo> <msup><mi>x</mi><mn>2</mn></msup>
                <mo>+</mo><mi>b</mi><mo>&InvisibleTimes;</mo><mi>x</mi>
                <mo>+</mo><mi>c</mi>
            </mrow>
        </math>";

        let mut reader = MathMl::new();
        assert!(reader.read(content).is_ok());

        let math = reader.math;
        assert_eq!(1, math.len());

        let element = &math[0];
        assert_eq!(9, element.get_elements().len());

        let sub_elements = element.get_elements();

        let sub_element = &sub_elements[0];
        assert_eq!(CommonElement::Identifier(Identifier::new(String::from("a"))), *sub_element);

        let sub_element = &sub_elements[1];
        assert_eq!(CommonElement::Operator(Operator::new(String::from("InvisibleTimes"))), *sub_element);

        let sub_element = &sub_elements[2];
        assert_eq!(CommonElement::Superscript(
            Superscript::new(
                &Identifier::new(String::from("x")),
                &Identifier::new(String::from("2")))
            ),
            *sub_element
        );

        let sub_element = &sub_elements[3];
        assert_eq!(CommonElement::Operator(Operator::new(String::from("+"))), *sub_element);

        let sub_element = &sub_elements[4];
        assert_eq!(CommonElement::Identifier(Identifier::new(String::from("b"))), *sub_element);

        let sub_element = &sub_elements[5];
        assert_eq!(CommonElement::Operator(Operator::new(String::from("InvisibleTimes"))), *sub_element);

        let sub_element = &sub_elements[6];
        assert_eq!(CommonElement::Identifier(Identifier::new(String::from("x"))), *sub_element);

        let sub_element = &sub_elements[7];
        assert_eq!(CommonElement::Operator(Operator::new(String::from("+"))), *sub_element);

        let sub_element = &sub_elements[8];
        assert_eq!(CommonElement::Identifier(Identifier::new(String::from("c"))), *sub_element);
    }
}
