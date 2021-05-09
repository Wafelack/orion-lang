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
    builtins: Vec<String>,
}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        let mut to_ret = Self {
            input,
            output: Bytecode::new(),
            load_history: vec![],
            builtins: vec![],
        };
        to_ret.register_builtin("+");
        to_ret.register_builtin("dbg");
        to_ret
    }
    fn register_builtin(&mut self, name: impl ToString) {
        self.builtins.push(name.to_string())
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
    fn declare(
        &mut self,
        name: impl ToString,
        mut symbols: Vec<String>,
        ) -> Result<(u16, Vec<String>)> {
        if self.output.symbols.len() >= u16::MAX as usize {
            error!("Too much symbols are declared.")
        } else {
            Ok((
                    if symbols.contains(&name.to_string()) {
                        symbols.iter().position(|s| s == &name.to_string()).unwrap()
                    } else {
                        symbols.push(name.to_string());
                        symbols.len()
                    } as u16,
                    symbols,
                    ))
        }
    }
    fn load_file(
        &mut self,
        fname: impl ToString,
        mut symbols: Vec<String>,
        ) -> Result<(Vec<OpCode>, Vec<String>)> {
        let fname = fname.to_string();
        if self.load_history.contains(&fname) {
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
                                let to_ret = self.compile_expr(e, symbols.clone())?;
                                symbols = to_ret.1;
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
        mut symbols: Vec<String>,
        ) -> Result<(Vec<OpCode>, Vec<String>)> {
        match expr {
            Expr::Literal(lit) => Ok((
                    vec![(OpCode::LoadConst(self.register_constant(lit)?))],
                    symbols,
                    )),
            Expr::Var(name) => {
                if !symbols.contains(&name) {
                    error!("Variable not in scope: {}.", name)
                } else {
                    let (idx, symbols) = self.declare(name, symbols)?;
                    Ok((vec![OpCode::LoadSym(idx)], symbols))
                }
            }
            Expr::Load(files) => {
                let lib_link = match env::var("ORION_LIB") {
                    Ok(v) => v,
                    Err(_) => return error!("ORION_LIB variable does not exist.")
                };

                Ok((files.into_iter().map(|file| {
                    let lib_path = format!("{}/{}", lib_link, file);
                    let fname = if Path::new(&lib_path).exists() {
                        Ok(lib_path)
                    } else if Path::new(&file).exists() {
                        Ok(file)
                    } else {
                        error!("File not found: {}.", file)
                    }?;

                    let to_ret = self.load_file(fname, symbols.clone())?;
                    symbols = to_ret.1;
                    Ok(to_ret.0)
                }).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().flatten().collect::<Vec<OpCode>>(), symbols))
            }
            Expr::Def(name, expr) => {
                let (idx, symbols) = self.declare(name, symbols)?;
                let (mut to_ret, symbols) = self.compile_expr(*expr, symbols)?;
                to_ret.push(OpCode::Def(idx));
                Ok((to_ret, symbols))
            }
            Expr::Call(func, args) => {
                let (mut to_ret, mut symbols) = self.compile_expr(*func, symbols)?;
                let argc = args.len() as u16;
                to_ret.extend(args.into_iter()
                    .map(|a| {
                        let (opcodes, syms) = self.compile_expr(a, symbols.clone())?;
                        symbols = syms;
                        Ok(opcodes)
                    }).collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>());
                to_ret.push(OpCode::Call(argc));
                Ok((to_ret, symbols))
            }
            Expr::Lambda(args, body) => {
                let chunk_symbols = args.iter().map(|a| {
                    let (idx, syms) = self.declare(a, symbols.clone())?;
                    symbols = syms;
                    Ok(idx)
                }).collect::<Result<Vec<_>>>()?;
                let run_with = symbols.iter().enumerate().map(|(idx, sym)| {
                    match chunk_symbols.iter().position(|id| *id as usize == idx) {
                        Some(arg_i) => args[arg_i].to_string(),
                        None => sym.to_string(),
                    }
                }).collect::<Vec<String>>();
                let (chunk_instructions, symbols) = self.compile_expr(*body, run_with)?;
                self.output.chunks.push(Chunk {
                    instructions: chunk_instructions,
                    symbols: chunk_symbols,
                });
                Ok((vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1)], symbols))
            }
            Expr::Builtin(name, args) => {
                let idx = self.builtins.iter().position(|builtin| builtin == &name).unwrap();
                let argc = args.len();
                let mut to_ret = args.into_iter().map(|arg| {
                    let (compiled, new_syms) = self.compile_expr(arg, symbols.clone())?;
                    symbols = new_syms;
                    Ok(compiled)
                }).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().flatten().collect::<Vec<OpCode>>();
                to_ret.push(OpCode::Builtin(idx as u8, argc as u8));
                Ok((to_ret, symbols))
            }
            _ => todo!(),
        }
    }
    pub fn compile(&mut self) -> Result<Bytecode> {
        let mut symbols = vec![];
        for expr in self.input.clone() {
            let (to_push, new_symbols) = self.compile_expr(expr, symbols)?;
            symbols = new_symbols;
            self.output.instructions.extend(to_push);
        }

        self.output.symbols = symbols;

        Ok(self.output.clone())
    }
}
