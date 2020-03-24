//!
//! The place expression translator.
//!

use std::convert::TryFrom;

use crate::semantic::analyzer::expression::hint::Hint as TranslationHint;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::Place;
use crate::semantic::element::value::Value;
use crate::semantic::element::Element;
use crate::semantic::error::Error;

pub struct Translator {}

impl Translator {
    pub fn translate(place: &Place, translation_hint: TranslationHint) -> Result<Element, Error> {
        match translation_hint {
            TranslationHint::ValueExpression => Value::try_from(&place.r#type)
                .map(Element::Value)
                .map_err(ElementError::Value)
                .map_err(|error| Error::Element(place.location, error)),
            _ => Ok(Element::Place(place.to_owned())),
        }
    }
}
