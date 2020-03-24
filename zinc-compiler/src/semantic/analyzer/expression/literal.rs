//!
//! The literal semantic analyzer.
//!

use std::convert::TryFrom;

use crate::generator::expression::operand::constant::Constant as GeneratorConstant;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::Element;
use crate::semantic::error::Error;
use crate::syntax::BooleanLiteral;
use crate::syntax::IntegerLiteral;
use crate::syntax::StringLiteral;

pub struct Analyzer {}

impl Analyzer {
    pub fn boolean(literal: BooleanLiteral) -> Result<(Element, Option<GeneratorConstant>), Error> {
        let constant = Constant::from(literal);
        let intermediate = GeneratorConstant::try_from_semantic(&constant);
        let element = Element::Constant(constant);

        Ok((element, intermediate))
    }

    pub fn integer(literal: IntegerLiteral) -> Result<(Element, Option<GeneratorConstant>), Error> {
        let location = literal.location;

        let constant = IntegerConstant::try_from(&literal)
            .map(Constant::Integer)
            .map_err(|error| {
                Error::Element(
                    location,
                    ElementError::Constant(ConstantError::Integer(error)),
                )
            })?;
        let intermediate = GeneratorConstant::try_from_semantic(&constant);
        let element = Element::Constant(constant);

        Ok((element, intermediate))
    }

    pub fn string(literal: StringLiteral) -> Result<Element, Error> {
        Ok(Element::Constant(Constant::String(literal.data.value)))
    }
}
