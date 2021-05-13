use crate::{
    bytecode::{Bytecode, Chunk, OpCode},
    error,
    lexer::Lexer,
    parser::{Expr, Literal, Parser},
    OrionError, Result,
};
use std::{env, fs, path::Path};
pub struct Compiler {
    input: Vec<Expr>,
    output: Bytecode,
    load_history: Vec<String>,
    builtins: Vec<(String, bool)>, // (name, impure?)
    constructors: Vec<String>,
}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        let mut to_ret = Self {
            input,
            constructors: vec![],
            output: Bytecode::new(),
            load_history: vec![],
            builtins: vec![],
        };
        to_ret.register_builtin("+", false);
        to_ret.register_builtin("dbg", true);
        to_ret.register_builtin("unquote", false);
        to_ret
    }
    fn register_builtin(&mut self, name: impl ToString, impure: bool) {
        self.builtins.push((name.to_string(), impure))
    }
    fn register_constant(&mut self, constant: Literal) -> Result<u16> {
        if !self.output.constants.contains(&constant) {
            self.output.constants.push(constant.clone());
        }
        if self.output.constants.len() > u16::MAX as usize {
            error!("Too much constants are used.")
        } else {
            Ok(self
                .output
                .constants
                .iter()
                .position(|c| c == &constant)
                .unwrap() as u16)
        }
    }
    fn register_constructor(&mut self, name: impl ToString, contained_amount: u8) -> Result<()> {
        let name = name.to_string();
        if self.constructors.contains(&name) {
            error!(
                "Enum Variant {} has already been defined (Index 0x{:04x})",
                &name,
                self.constructors
                    .iter()
                    .position(|var| var.to_string() == name)
                    .unwrap()
            )
        } else {
            self.constructors.push(name);
            self.output.constructors.push(contained_amount);
            Ok(())
        }
    }
    fn get_constructor(&self, name: impl ToString) -> Result<(u8, u16)> {
        let name = name.to_string();
        if self.constructors.contains(&name) {
            let idx = self
                .constructors
                .iter()
                .position(|variant| name == variant.to_string())
                .unwrap();
            Ok((self.output.constructors[idx], idx as u16))
        } else {
            error!("Enum Variant {} does not exist.", name)
        }
    }
    fn declare(
        &mut self,
        name: impl ToString,
        mut symbols: Vec<(String, bool)>,
        impure: bool,
    ) -> Result<(u16, Vec<(String, bool)>)> {
        if self.output.symbols.len() >= u16::MAX as usize {
            error!("Too much symbols are declared.")
        } else {
            Ok((
                if symbols.contains(&(name.to_string(), impure))
                    || symbols.contains(&(name.to_string(), !impure))
                {
                    symbols
                        .iter()
                        .position(|s| s.0 == name.to_string())
                        .unwrap()
                } else {
                    symbols.push((name.to_string(), impure));
                    symbols.len()
                } as u16,
                symbols,
            ))
        }
    }
    fn load_file(
        &mut self,
        fname: impl ToString,
        mut symbols: Vec<(String, bool)>,
    ) -> Result<(Vec<OpCode>, Vec<(String, bool)>)> {
        let fname = fname.to_string();
        if self.load_history.contains(&fname) {
            // Avoid error-prone reloading if file has already been loaded.
            Ok((vec![], symbols))
        } else {
            self.load_history.push(fname.clone());
            match fs::read_to_string(&fname) {
                Ok(content) => {
                    let tokens = Lexer::new(content).proc_tokens()?;
                    let expressions = Parser::new(tokens).parse()?;

                    Ok((
                        expressions
                            .into_iter()
                            .map(|e| {
                                let to_ret = self.compile_expr(e, symbols.clone(), true)?;
                                symbols = to_ret.1; // Update symbols.
                                Ok(to_ret.0)
                            })
                            .collect::<Result<Vec<Vec<OpCode>>>>()?
                            .into_iter()
                            .flatten()
                            .collect::<Vec<OpCode>>(),
                        symbols,
                    ))
                }
                Err(e) => error!("Failed to read file: {}: {}.", fname, e),
            }
        }
    }
    fn compile_expr(
        &mut self,
        expr: Expr,
        mut symbols: Vec<(String, bool)>,
        impure: bool,
    ) -> Result<(Vec<OpCode>, Vec<(String, bool)>)> {
        match expr {
            Expr::Literal(lit) => Ok((
                vec![(OpCode::LoadConst(self.register_constant(lit)?))],
                symbols,
            )),
            Expr::Var(name) => {
                if !symbols.contains(&(name.clone(), impure)) {
                    if impure && symbols.contains(&(name.clone(), false)) {
                        let (idx, symbols) = self.declare(name, symbols, impure)?;
                        Ok((vec![OpCode::LoadSym(idx)], symbols))
                    } else if !impure && symbols.contains(&(name.clone(), true)) {
                        error!(
                            "Impure function used out of an `impure` declaration: {}",
                            name
                        )
                    } else {
                        error!("Variable not in scope: {}.", name)
                    }
                } else {
                    let (idx, symbols) = self.declare(name, symbols, impure)?;
                    Ok((vec![OpCode::LoadSym(idx)], symbols))
                }
            }
            Expr::Load(files) => {
                let lib_link = match env::var("ORION_LIB") {
                    Ok(v) => v,
                    Err(_) => return error!("ORION_LIB variable does not exist."),
                };

                Ok((
                    files
                        .into_iter()
                        .map(|file| {
                            let lib_path = format!("{}/{}", lib_link, file);
                            let fname = if Path::new(&lib_path).exists() {
                                Ok(lib_path)
                            } else if Path::new(&file).exists() {
                                Ok(file)
                            } else {
                                error!("File not found: {}.", file)
                            }?;

                            let to_ret = self.load_file(fname, symbols.clone())?;
                            symbols = to_ret.1; // Update symbols.
                            Ok(to_ret.0)
                        })
                        .collect::<Result<Vec<Vec<OpCode>>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<OpCode>>(),
                    symbols,
                ))
            }
            Expr::Def(name, expr, purity) => {
                let (idx, symbols) = self.declare(name, symbols, purity)?;
                let (mut to_ret, symbols) = self.compile_expr(*expr, symbols, purity)?; // Update symbols.
                to_ret.push(OpCode::Def(idx));
                Ok((to_ret, symbols))
            }
            Expr::Call(func, args) => {
                let (mut to_ret, mut symbols) = self.compile_expr(*func, symbols, impure)?; // The λ to execute.
                let argc = args.len() as u16;
                to_ret.extend(
                    // Push arguments onto the stack, and keep the amount in order to pop all the arguments.
                    args.into_iter()
                        .map(|a| {
                            let (opcodes, syms) = self.compile_expr(a, symbols.clone(), impure)?;
                            symbols = syms; // Update symbols.
                            Ok(opcodes)
                        })
                        .collect::<Result<Vec<Vec<OpCode>>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<OpCode>>(),
                );
                to_ret.push(OpCode::Call(argc));
                Ok((to_ret, symbols))
            }
            Expr::Lambda(args, body) => {
                let args_reference = args
                    .iter()
                    .map(|a| {
                        let (idx, syms) = self.declare(a, symbols.clone(), false)?;
                        symbols = syms;
                        Ok(idx)
                    })
                    .collect::<Result<Vec<_>>>()?; // Position in the symbol table for each arg.
                let run_with = symbols
                    .iter()
                    .enumerate()
                    .map(|(idx, sym)| {
                        match args_reference.iter().position(|id| *id as usize == idx) {
                            // Check if the current symbol is part of the arguments
                            // If it is, then replace it by the argument's value, else
                            // use the value that is already in the table.
                            Some(arg_i) => (args[arg_i].to_string(), false),
                            None => sym.clone(),
                        }
                    })
                    .collect::<Vec<(String, bool)>>();
                let (chunk_instructions, symbols) = self.compile_expr(*body, run_with, impure)?;
                self.output.chunks.push(Chunk {
                    instructions: chunk_instructions,
                    reference: args_reference,
                });
                Ok((
                    vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1)],
                    symbols,
                ))
            }
            Expr::Builtin(name, args) => {
                let idx = self
                    .builtins
                    .iter()
                    .position(|builtin| builtin.0 == name)
                    .unwrap();
                let impure_builtin = self.builtins[idx as usize].1;
                if !impure && impure_builtin {
                    return error!("Impure builtin used out of an `impure` function: {}.", name);
                }
                let argc = args.len();
                let mut to_ret = args
                    .into_iter()
                    .map(|arg| {
                        let (compiled, new_syms) =
                            self.compile_expr(arg, symbols.clone(), impure)?;
                        symbols = new_syms; // Update symbols.
                        Ok(compiled)
                    })
                    .collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>();
                to_ret.push(OpCode::Builtin(idx as u8, argc as u8));
                Ok((to_ret, symbols))
            }
            Expr::Enum(_, constructors) => {
                constructors
                    .into_iter()
                    .map(|(k, v)| self.register_constructor(k, v))
                    .collect::<Result<()>>()?;
                Ok((vec![], symbols))
            }
            Expr::Constr(name, contained) => {
                let (amount, idx) = self.get_constructor(&name)?;
                if amount != contained.len() as u8 {
                    error!(
                        "Enum Constructor {} takes {} values, but {} values were given.",
                        name,
                        amount,
                        contained.len()
                    )
                } else {
                    let values = contained
                        .into_iter()
                        .map(|expr| {
                            let (compiled, new_syms) =
                                self.compile_expr(expr, symbols.clone(), impure)?;
                            symbols = new_syms;
                            Ok(compiled)
                        })
                        .collect::<Result<Vec<Vec<OpCode>>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<OpCode>>();
                    let mut to_ret = vec![OpCode::Constructor(idx, values.len() as u16)];
                    to_ret.extend(values);
                    Ok((to_ret, symbols))
                }
            }
            Expr::Quote(expr) => {
                let (body, symbols) = self.compile_expr(*expr, symbols, impure)?;
                let mut to_ret = vec![OpCode::Quote(body.len() as u16)];
                to_ret.extend(body);
                Ok((to_ret, symbols))
            }
            Expr::Tuple(vals) => {
                let vals_len = vals.len();
                let values = vals
                    .into_iter()
                    .map(|expr| {
                        let (compiled, new_syms) =
                            self.compile_expr(expr, symbols.clone(), impure)?;
                        symbols = new_syms;
                        Ok(compiled)
                    })
                    .collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>();
                let mut to_ret = vec![OpCode::Tuple(values.len() as u16, vals_len as u16)];
                to_ret.extend(values);
                Ok((to_ret, symbols))
            }
            _ => todo!(),
        }
    }
    pub fn compile(&mut self) -> Result<Bytecode> {
        let mut symbols = vec![];
        for expr in self.input.clone() {
            let (to_push, new_symbols) = self.compile_expr(expr, symbols, true)?;
            symbols = new_symbols;
            self.output.instructions.extend(to_push);
        }

        self.output.symbols = symbols
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<String>>();

        Ok(self.output.clone())
    }
}
