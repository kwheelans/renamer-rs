# Renamer-rs
[![crates.io](https://img.shields.io/crates/v/renamer-rs)](https://crates.io/crates/renamer-rs)
[![Released API docs](https://docs.rs/renamer-rs/badge.svg)](https://docs.rs/renamer-rs/)
[![MIT licensed](https://img.shields.io/crates/l/renamer-rs)](./LICENSE)

 A library to process and rename files or text



 ## Example
 ```rust,no_run
 use renamer_rs::{Renamed, Delimiter, Selector, Format, Error, InputType};

 fn run(delimiter: Delimiter, selector: Selector, format: Format, input: InputType) -> Result<Vec<Box<dyn Renamed>>, Error> { 
     let processor = renamer_rs::ProcessorBuilder::new(format)
         .delimiter(delimiter)
         .selector(selector)
         .input(input);
     processor.process()
 }
```