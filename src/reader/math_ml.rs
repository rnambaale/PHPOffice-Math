
use crate::elements::operator::Operator;
use crate::elements::CommonElement;
use crate::{elements::identifier::Identifier, reader::reader_interface::ReaderInterface};
use crate::elements::row::Row;
use xml::reader::{EventReader, XmlEvent};

#[allow(dead_code)]
pub struct MathMl {
    math: Vec<Row>,
    // dom: String,//
    // xpath: String,
}

#[derive(Debug)]
enum Common {
    Row(Row),
    Identifier(Identifier),
    Operator(Operator),
}

#[allow(dead_code)]
impl ReaderInterface for MathMl {
    fn read(&mut self, content: &str) -> Result<(), String> {
        // let content = content.replace("&InvisibleTimes;", "<mchar name=\"InvisibleTimes\"/>");
        let content = content.replace("&InvisibleTimes;", "InvisibleTimes");

        let mut elements_stack: Vec<Common> = Vec::new();

        let parser = EventReader::from_str(&content);

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    // println!("{:spaces$}+{name}", "", spaces = depth * 2);
                    let element_name = name.local_name;
                    // dbg!("StartElement: ", format!("<{}>", &element_name));
                    println!("StartElement: <{}>", &element_name);

                    match element_name.as_str() {
                        "mrow" => elements_stack.push(Common::Row(Row::new())),
                        "mi" => elements_stack.push(Common::Identifier(Identifier::new(String::from("")))),
                        "mo" => elements_stack.push(Common::Operator(Operator::new(String::from("")))),
                        _ => {}
                    }

                }

                Ok(XmlEvent::Characters(text)) => {
                    println!("  Characters {}", text);
                    // if let Some(element) = elements_stack.last_mut() {
                    //     element.value = text;
                    // }
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
                    // println!("{:spaces$}-{name}", "", spaces = depth * 2);
                    // dbg!("EndElement");
                    let element_name = name.local_name;
                    // dbg!("EndElement </{}>", &element_name);
                    println!("EndElement </{}>", &element_name);
                    match element_name.as_str() {
                        "mi" => {
                            if let Some(Common::Identifier(element)) = elements_stack.pop() {
                                // println!("element: {:#?}", element);
                                let parent = elements_stack.last_mut().unwrap();
                                // println!("parent: {:#?}", parent);
                                if let Common::Row(row) = parent {
                                    row.add(CommonElement::Identifier(element.clone()));
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
                        // "mrow" => {
                        //     if let Some(Common::Identifier(element)) = elements_stack.pop() {
                        //         let parent = elements_stack.last_mut().unwrap();
                        //         if let Common::Row(row) = parent {
                        //             row.add(&element);
                        //         }
                        //     }
                        // }
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

        // self.math = Row::new();
        // if let Some(Common::Row(row)) = elements_stack.pop() {
        //     self.math = (row);
        // }
        // println!("{:#?}", elements_stack);
        let mut math: Vec<Row> = Vec::new();
        for item in &elements_stack {
            match item {
                Common::Row(row) => math.push(row.clone()),
                _ => {}
            }
            // if let Common::Row(row) = item {
            //     self.math.push(row.clone());
            // }
        }
        self.math = math;
        // match elements_stack.pop() {
        //     Some(row) => {
        //         self.math = row;
        //     }
        //     None => {}
        // }
        // self.math.add(&elements_stack);
        // self.math = elements_stack;

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

        // /** @var Element\Superscript $subElement */
        // $subElement = $subElements[2];
        // $this->assertInstanceOf(Element\Superscript::class, $subElements[2]);

        // /** @var Element\Identifier $base */
        // $base = $subElement->getBase();
        // $this->assertInstanceOf(Element\Identifier::class, $base);
        // $this->assertEquals('x', $base->getValue());

        // /** @var Element\Numeric $superscript */
        // $superscript = $subElement->getSuperscript();
        // $this->assertInstanceOf(Element\Numeric::class, $superscript);
        // $this->assertEquals(2, $superscript->getValue());
    }
}
