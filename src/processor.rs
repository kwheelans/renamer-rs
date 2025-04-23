pub mod delimiter;
pub mod extractor;
pub mod format;
pub mod rename;
pub mod replacer;
pub mod selector;
pub mod trim;

use crate::error::Error;
use crate::processor::delimiter::Delimiter;
use crate::processor::extractor::Extractor;
use crate::processor::format::Format;
use crate::processor::rename::{FileRenamer, Renamed, Renamer, filename_as_string};
use crate::processor::replacer::Replacer;
use crate::processor::selector::Selector;
use crate::processor::trim::Trim;
use log::trace;
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProcessorBuilder {
    delimiters: Vec<Delimiter>,
    extractors: Vec<Extractor>,
    selectors: Vec<Selector>,
    format: Format,
    trims: Vec<Trim>,
    replacers: Vec<Replacer>,
    files: HashSet<PathBuf>,
}

impl ProcessorBuilder {
    pub fn new(format: Format) -> Self {
        Self {
            delimiters: Vec::new(),
            extractors: Vec::new(),
            selectors: Vec::new(),
            format,
            trims: Vec::new(),
            replacers: Vec::new(),
            files: HashSet::new(),
        }
    }

    pub fn delimiter(mut self, delimiter: Delimiter) -> Self {
        self.delimiters.push(delimiter);
        self
    }

    pub fn delimiters(mut self, delimiters: Vec<Delimiter>) -> Self {
        self.delimiters.extend(delimiters);
        self
    }

    pub fn file(mut self, file: PathBuf) -> Self {
        self.files.insert(file);
        self
    }

    pub fn files(mut self, files: Vec<PathBuf>) -> Self {
        self.files.extend(files);
        self
    }

    pub fn selector(mut self, selector: Selector) -> Self {
        self.selectors.push(selector);
        self
    }

    pub fn selectors(mut self, selectors: Vec<Selector>) -> Self {
        self.selectors.extend(selectors);
        self
    }

    pub fn extractor(mut self, extractor: Extractor) -> Self {
        self.extractors.push(extractor);
        self
    }

    pub fn extractors(mut self, extractors: Vec<Extractor>) -> Self {
        self.extractors.extend(extractors);
        self
    }

    pub fn trim(mut self, trim: Trim) -> Self {
        self.trims.push(trim);
        self
    }

    pub fn trims(mut self, trims: Vec<Trim>) -> Self {
        self.trims.extend(trims);
        self
    }

    pub fn replacer(mut self, trim: Replacer) -> Self {
        self.replacers.push(trim);
        self
    }

    pub fn replacers(mut self, trims: Vec<Replacer>) -> Self {
        self.replacers.extend(trims);
        self
    }

    pub fn process(&self) -> Result<Vec<Box<dyn Renamed>>, Error> {
        let mut renamed = Vec::new();
        renamed.extend(self.process_files()?);

        Ok(renamed)
    }

    fn process_files(&self) -> Result<Vec<Box<dyn Renamed>>, Error> {
        let mut renamed = Vec::new();
        for file in self.files.iter() {
            let filename = filename_as_string(file.as_path());
            let extracted = self.process_extractors(filename.as_str());
            let filename = vec![filename];
            let segments = self.process_delimiters(filename.as_slice());
            let segments = self.process_trims(segments);
            let segments = self.process_replacers(segments);
            let selected = self.process_selectors(segments.as_slice());
            renamed.push(
                FileRenamer::new(file, segments, selected, extracted, self.format.clone()).rename(),
            );
        }
        Ok(renamed)
    }

    fn process_delimiters<S: AsRef<str>>(&self, value: &[S]) -> Vec<String> {
        let mut output = Vec::new();
        for delimiter in &self.delimiters {
            for seg in value {
                let mut segments = delimiter.split(seg);
                output.append(&mut segments);
            }
            trace!(
                "After Delimiter: |{}| --- Output Segments Count: {}",
                delimiter,
                output.len()
            )
        }
        output
    }

    fn process_selectors(&self, segments: &[String]) -> Vec<Option<String>> {
        self.selectors
            .iter()
            .map(|s| s.match_segment(segments))
            .collect()
    }
    fn process_extractors<S: AsRef<str>>(&self, value: S) -> Vec<Option<String>> {
        self.extractors
            .iter()
            .map(|e| e.extract(value.as_ref()))
            .collect()
    }
    
    fn process_trims(&self, segments: Vec<String>) -> Vec<String> {
        let mut output = segments;
        for t in self.trims.as_slice() {
            output = t.trim_slice(output.as_slice())
        }
        output
    }
    
    fn process_replacers(&self, segments: Vec<String>) -> Vec<String> {
        let mut output = segments;
        for r in self.replacers.as_slice() {
            output = r.replace_slice(output.as_slice())
        }
        output
    }
}
