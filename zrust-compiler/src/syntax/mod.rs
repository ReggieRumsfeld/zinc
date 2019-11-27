//!
//! The syntax parser.
//!

mod error;
mod parser;
mod tests;
mod tree;

pub use self::error::Error;
pub use self::parser::AccessOperandParser;
pub use self::parser::AddSubOperandParser;
pub use self::parser::AndOperandParser;
pub use self::parser::ArrayExpressionParser;
pub use self::parser::BlockExpressionParser;
pub use self::parser::CastingOperandParser;
pub use self::parser::ComparisonOperandParser;
pub use self::parser::ConditionalExpressionParser;
pub use self::parser::EnumStatementParser;
pub use self::parser::ExpressionListParser;
pub use self::parser::ExpressionParser;
pub use self::parser::FieldListParser;
pub use self::parser::FieldParser;
pub use self::parser::FnStatementParser;
pub use self::parser::InnerStatementParser;
pub use self::parser::LetStatementParser;
pub use self::parser::LoopStatementParser;
pub use self::parser::MatchExpressionParser;
pub use self::parser::ModStatementParser;
pub use self::parser::MulDivRemOperandParser;
pub use self::parser::OrOperandParser;
pub use self::parser::OuterStatementParser;
pub use self::parser::Parser;
pub use self::parser::PathExpressionParser;
pub use self::parser::PatternParser;
pub use self::parser::StructStatementParser;
pub use self::parser::StructureExpressionParser;
pub use self::parser::TupleExpressionParser;
pub use self::parser::TypeParser;
pub use self::parser::TypeStatementParser;
pub use self::parser::UseStatementParser;
pub use self::parser::VariantListParser;
pub use self::parser::VariantParser;
pub use self::parser::XorOperandParser;
pub use self::tree::ArrayExpression;
pub use self::tree::ArrayExpressionBuilder;
pub use self::tree::BlockExpression;
pub use self::tree::BlockExpressionBuilder;
pub use self::tree::CircuitProgram;
pub use self::tree::ConditionalExpression;
pub use self::tree::ConditionalExpressionBuilder;
pub use self::tree::EnumStatement;
pub use self::tree::EnumStatementBuilder;
pub use self::tree::Expression;
pub use self::tree::ExpressionBuilder;
pub use self::tree::ExpressionElement;
pub use self::tree::ExpressionObject;
pub use self::tree::ExpressionOperand;
pub use self::tree::ExpressionOperator;
pub use self::tree::Field;
pub use self::tree::FieldBuilder;
pub use self::tree::FnStatement;
pub use self::tree::FnStatementBuilder;
pub use self::tree::Identifier;
pub use self::tree::IdentifierBuilder;
pub use self::tree::InnerStatement;
pub use self::tree::LetStatement;
pub use self::tree::LetStatementBuilder;
pub use self::tree::Literal;
pub use self::tree::LoopStatement;
pub use self::tree::LoopStatementBuilder;
pub use self::tree::MatchExpression;
pub use self::tree::MatchExpressionBuilder;
pub use self::tree::ModStatement;
pub use self::tree::ModStatementBuilder;
pub use self::tree::OuterStatement;
pub use self::tree::Pattern;
pub use self::tree::PatternBuilder;
pub use self::tree::PatternVariant;
pub use self::tree::StructStatement;
pub use self::tree::StructStatementBuilder;
pub use self::tree::StructureExpression;
pub use self::tree::StructureExpressionBuilder;
pub use self::tree::TupleExpression;
pub use self::tree::TupleExpressionBuilder;
pub use self::tree::Type;
pub use self::tree::TypeBuilder;
pub use self::tree::TypeStatement;
pub use self::tree::TypeStatementBuilder;
pub use self::tree::TypeVariant;
pub use self::tree::UseStatement;
pub use self::tree::UseStatementBuilder;
pub use self::tree::Variant;
pub use self::tree::VariantBuilder;

static PANIC_ALL_TYPE_CASES_ARE_CHECKED_ABOVE: &str = "All the type cases are covered above";
static PANIC_ALL_TYPE_KEYWORDS_ARE_CHECKED_ABOVE: &str = "All the type keywords are covered above";
static PANIC_ALL_TUPLE_CASES_ARE_COVERED_ABOVE: &str = "All the tuple cases are covered above";
static PANIC_BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";
static PANIC_VALUE_ALWAYS_EXISTS: &str = "The value always exists at this point";
