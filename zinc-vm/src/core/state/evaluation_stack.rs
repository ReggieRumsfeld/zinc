//!
//! The VM state evaluation stack.
//!

use std::fmt;

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::state::cell::Cell;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::Scalar;
use crate::Engine;

#[derive(Debug)]
pub struct EvaluationStack<E: Engine> {
    stack: Vec<Vec<Cell<E>>>,
}

impl<E: Engine> Default for EvaluationStack<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Engine> EvaluationStack<E> {
    pub fn new() -> Self {
        Self {
            stack: vec![vec![]],
        }
    }

    pub fn push(&mut self, value: Cell<E>) -> Result<(), RuntimeError> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                RuntimeError::InternalError("Evaluation stack root frame missing".into())
            })?
            .push(value);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Cell<E>, RuntimeError> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                RuntimeError::InternalError("Evaluation stack root frame missing".into())
            })?
            .pop()
            .ok_or_else(|| MalformedBytecode::StackUnderflow.into())
    }

    pub fn fork(&mut self) {
        self.stack.push(vec![]);
    }

    pub fn merge<CS>(&mut self, mut cs: CS, condition: &Scalar<E>) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let else_case = self.stack.pop().ok_or_else(|| {
            RuntimeError::InternalError("Evaluation stack root frame missing".into())
        })?;
        let then_case = self.stack.pop().ok_or_else(|| {
            RuntimeError::InternalError("Evaluation stack root frame missing".into())
        })?;

        if then_case.len() != else_case.len() {
            return Err(MalformedBytecode::BranchStacksDoNotMatch.into());
        }

        for (index, (main_value, else_value)) in
            then_case.into_iter().zip(else_case.into_iter()).enumerate()
        {
            match (main_value, else_value) {
                (Cell::Value(main_value), Cell::Value(else_value)) => {
                    let merged = gadgets::conditional_select::conditional_select(
                        cs.namespace(|| format!("merge {}", index)),
                        condition,
                        &main_value,
                        &else_value,
                    )?;

                    self.push(Cell::Value(merged))?;
                }
            }
        }

        Ok(())
    }

    pub fn revert(&mut self) -> Result<(), RuntimeError> {
        self.stack.pop().ok_or(MalformedBytecode::StackUnderflow)?;
        Ok(())
    }
}

impl<E: Engine> fmt::Display for EvaluationStack<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Evaluation stack:")?;

        for frame in self.stack.iter().rev() {
            for cell in frame.iter().rev() {
                let Cell::Value(value) = cell;
                writeln!(f, "\t{}", value)?;
            }
        }

        Ok(())
    }
}
