/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{vm::{VM, Value}, OrionError, error, Result};

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn add(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs + rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs + rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn sub(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs - rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs - rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn mul(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs * rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs * rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn div(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs / rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs / rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn neg(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Integer(val) => Ok(Value::Integer(-val)),
            Value::Single(val) => Ok(Value::Single(-val)),
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", val),
        }
    }
    pub fn cos(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.cos())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
    pub fn sin(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.sin())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
    pub fn tan(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.tan())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
    pub fn acos(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.acos())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
    pub fn asin(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.asin())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
    pub fn atan(&mut self) -> Result<Value> {
        let val = self.pop()?;

        match val {
            Value::Single(val) => Ok(Value::Single(val.atan())),
            _ => error!(=> "Expected a Single, found a {:?}.", val),
        }
    }
}
