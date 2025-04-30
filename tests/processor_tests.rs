use regex::Regex;
use renamer_rs::{Delimiter, Format, InputType, ProcessorBuilder, Selector, TextInput};

const TEXT_INPUT_1: &str = "Some simple text input to be split into segments other1 other2";
const TEXT_INPUT_2: &str = "44343 $6556 text has other values [] {} ?<>";
const TEXT_INPUT_3: &str = "This is a title S03E04 - some other stuff.txt %e%";

#[test]
fn simple_text_input_single_delimiter() {
    let format = Format::new("%d1% --- %d9%").expect("Invalid Format");
    let delimiter = Delimiter::default();
    let processor = ProcessorBuilder::new(format)
        .delimiter(delimiter)
        .input(InputType::Text(TextInput::new(TEXT_INPUT_1)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_2)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_3)));

    let renamed = processor.process().expect("Unable to process input");
    assert_eq!(renamed.len(), 3);
    assert_eq!(renamed.first().unwrap().future(), "Some --- segments");
    assert_eq!(renamed.get(1).unwrap().future(), "44343 --- ?<>");
    assert_eq!(renamed.get(2).unwrap().future(), "This --- stuff.txt");
}

#[test]
fn simple_text_input_selector() {
    let format = Format::new("%s1%").expect("Invalid Format");
    let delimiter = Delimiter::default();
    let selector = Selector::new(None, Regex::new("^ot").unwrap());
    let processor = ProcessorBuilder::new(format)
        .delimiter(delimiter)
        .selector(selector)
        .input(InputType::Text(TextInput::new(TEXT_INPUT_1)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_2)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_3)));

    let renamed = processor.process().expect("Unable to process input");
    assert_eq!(renamed.len(), 3);
    assert_eq!(renamed.first().unwrap().future(), "other1");
    assert_eq!(renamed.get(1).unwrap().future(), "other");
    assert_eq!(renamed.get(2).unwrap().future(), "other");
}
