
use std::str::FromStr;

use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
pub enum FormError {
    ParsingError { id: String, t: String },
    IdNotPresent { id: String },
    CantConvertElementToInput { id: String },
    CantConvertFormatFromStr { text: String},
    CantConvertElementToTextArea { id: String },
}

impl std::fmt::Display for FormError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FormError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn get_by_id(id: &str, doc: &web_sys::Document) -> Result<web_sys::Element, FormError> {
    doc.get_element_by_id(id).ok_or(FormError::IdNotPresent{id: id.to_string()})
}

pub fn get_input(id: &str, doc: &web_sys::Document) -> Result<web_sys::HtmlInputElement, FormError> {
    match get_by_id(id, doc)?.dyn_into::<web_sys::HtmlInputElement>() {
	Ok(v) => Ok(v),
	Err(_) => Err(FormError::CantConvertElementToInput{id: id.to_string()}),
    }
}

pub fn get_text_area(id: &str, doc: &web_sys::Document) -> Result<web_sys::HtmlTextAreaElement, FormError> {
    match get_by_id(id, doc)?.dyn_into::<web_sys::HtmlTextAreaElement>() {
	Ok(v) => Ok(v),
	Err(_) => Err(FormError::CantConvertElementToTextArea{id: id.to_string()}),
    }
}

pub fn get_value<T>(id: &str, doc: &web_sys::Document) -> Result<T, FormError>
    where T: std::str::FromStr,
{
    match get_input(id, doc)?.value().parse::<T>() {
	Ok(number) => Ok(number),
	Err(_) => Err(FormError::ParsingError{id: id.to_string(), t: std::any::type_name::<T>().to_string()}),
    }
}

pub fn get_radio_value(id: &str, doc: &web_sys::Document) -> Result<Format, FormError> {
    let formats = doc.get_elements_by_name("format");
    for i in 0..formats.length() {
	if let Ok(format) = formats.get(i).unwrap().dyn_into::<web_sys::HtmlInputElement>() {
	    if format.checked() {
		return Format::from_str(&format.id());
	    }
	} else {
	    return Err(FormError::CantConvertElementToInput{id: id.to_string()});
	};
    }

    Err(FormError::ParsingError{id: id.to_string(), t: "generate::Format".to_string()})
}

#[derive(Debug)]
pub enum Format {
    None,
    Fasta,
    Fastq,
}

impl std::str::FromStr for Format {
    type Err = FormError;

    fn from_str(s: &str) -> Result<Format, Self::Err> {
	match s {
	    "none" => Ok(Format::None),
	    "fasta" => Ok(Format::Fasta),
	    "fastq" => Ok(Format::Fastq),
	    _ => Err(Self::Err::CantConvertFormatFromStr { text: s.to_string() }),
	}
    }
}
