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
    pub fn show(&mut self) -> Result<Value> {
        let popped = self.pop()?;
        Ok(Value::String(if let Value::String(s) = popped {
            format!("\"{}\"", s)
        } else {
            format!("{}", popped)
        }))
    }
}
