use regex::Regex;
use renamer_rs::{Delimiter, DelimiterType, Extractor, Format, InputType, ProcessorBuilder, Replacer, Selector, TextInput, Trim};

const TEXT_INPUT_1: &str = "Some simple text input to be split into segments other1 other2";
const TEXT_INPUT_2: &str = "44343 $6556 troubled troubadour other values [] {} ?<>";
const TEXT_INPUT_3: &str = "This is a title S03E04 - some other stuff.txt %e%";
const TRIM_TEXT_INPUT_1: &str = "smiddles ssthings beforesafter";

#[test]
fn simple_string_delimiter() {
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
fn simple_regex_delimiter() {
    let format = Format::new("%d1% --- %d9%").expect("Invalid Format");
    let delimiter = Delimiter::new(" ", DelimiterType::Regex).expect("unable to create delimiter");
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
fn simple_selector() {
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

#[test]
fn simple_extractor() {
    let format = Format::new("%e1%").expect("Invalid Format");
    let delimiter = Delimiter::default();
    let extractor = Extractor::new(None, Regex::new(r"t\S+").unwrap());
    let processor = ProcessorBuilder::new(format)
        .delimiter(delimiter)
        .extractor(extractor)
        .input(InputType::Text(TextInput::new(TEXT_INPUT_1)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_2)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_3)));

    let renamed = processor.process().expect("Unable to process input");
    assert_eq!(renamed.len(), 3);
    assert_eq!(renamed.first().unwrap().future(), "text");
    assert_eq!(renamed.get(1).unwrap().future(), "troubled");
    assert_eq!(renamed.get(2).unwrap().future(), "title");
}

#[test]
fn simple_trim() {
    let format = Format::new("%d1%-%d2%-%d3%").expect("Invalid Format");
    let delimiter = Delimiter::default();
    let trim = Trim::both("s");
    let processor = ProcessorBuilder::new(format)
        .delimiter(delimiter)
        .trim(trim)
        .input(InputType::Text(TextInput::new(TRIM_TEXT_INPUT_1)));


    let renamed = processor.process().expect("Unable to process input");
    assert_eq!(renamed.first().unwrap().future(), "middle-thing-beforesafter");

}

#[test]
fn simple_replace() {
    let format = Format::new("%d1% --- %d9%").expect("Invalid Format");
    let delimiter = Delimiter::default();
    let replace = Replacer::new(Regex::new("[Ss?]").unwrap(), "!");
    let processor = ProcessorBuilder::new(format)
        .delimiter(delimiter)
        .replacer(replace)
        .input(InputType::Text(TextInput::new(TEXT_INPUT_1)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_2)))
        .input(InputType::Text(TextInput::new(TEXT_INPUT_3)));

    let renamed = processor.process().expect("Unable to process input");
    assert_eq!(renamed.first().unwrap().future(), "!ome --- !egment!");
    assert_eq!(renamed.get(1).unwrap().future(), "44343 --- !<>");
    assert_eq!(renamed.get(2).unwrap().future(), "Thi! --- !tuff.txt");

}