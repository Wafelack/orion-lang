use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use std::io;

impl Interpreter {
    pub fn list(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        let mut first = "nil".to_owned();
        let mut toret = vec![];
        for i in 0..args.len() {
            if i == 0 {
                first = args[i].get_type();
            }

            if args[i].get_type() != first {
                return Err(
                    crate::error!("Invalid argument, expected", first, "found", (args[i].get_type()))
                )
            }


            toret.push(args[i].clone());
        }

        Ok(Value::List(toret))
    }

    pub fn input(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        use std::io::Write;

        Ok(if args.len() < 1 {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            Value::String(
                line
            )
        } else {
            let mut line = String::new();
            print!("{}", args[0]);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();
            Value::String(
                line
            )
        })
    }

    pub fn object(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        if args.len() % 2 != 0 {
            return Err(
                crate::error!("Invalid arguments, expected an even number of arguments, found", (args.len()))
            )
        }

        let mut toret: BTreeMap<String, Value> = BTreeMap::new();

        let mut i = 0;

        while i < args.len() {
            if let Value::String(s) = &args[i] {
                toret.insert(
                    s.to_owned(),
                    args[i + 1].to_owned(),
                );
                i += 2;
            } else {
                return Err(
                    crate::error!("Invalid argument, expected string, found", (args[i].get_type()))
                )
            }
        }

        Ok(
            Value::Object(toret)
        )
    }

    pub fn index(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        if let Value::Int(i) = &args[1] {
            let index = if *i > 0 {
                *i as usize
            } else {                
                return Err(
                    crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", i)
                )
            };

            let toret = if let Value::String(s) = &args[0] {
                if index >= s.len() {
                    return Err(
                        crate::error!("Index out of bounds, the length is", (s.len()), "but the index is", index)
                    )
                }
                Value::String(
                    s[index..index + 1].to_owned()
                )
            } else if let Value::List(l) = &args[0] {
                if index >= l.len() {
                    return Err(
                        crate::error!("Index out of bounds, the length is", (l.len()), "but the index is", index)
                    )
                }
                l[index].clone()
            } else {
                return Err(
                    crate::error!("Invalid argument, expected string or list, found", (&args[1].get_type()))
                )
            };

            Ok(toret)

        } else if let Value::String(s) = &args[1] {

            let toret = if let Value::Object(map) = &args[0] {
                if map.contains_key(s) {
                    map[s].clone()
                } else {
                    return Err(
                        crate::error!("Key `", s, "` not found in object `", (&args[0]),"`.")
                    )
                }
            } else {
                return Err(
                    crate::error!("Invalid argument, expected object, found", (&args[1].get_type()))
                )
            };

            Ok(toret)
        } else {
            return Err(
                crate::error!("Invalid argument, expected integer, found", (&args[1].get_type()))
            )
        }   
    }

    pub fn push(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2  && args.len() != 3{
            return Err(
                return Err(
                    crate::error!("Invalid number of arguments, expected 2|3, found", (args.len()))
                )
            )
        }

        if let Value::List(mut l) = args[0].clone() {
            if let Some(el) = l.last() {
                if args[1].get_type() != el.get_type() {
                    Err(
                        crate::error!("Type error: list is of type", (el.get_type()), "but element is", (args[1].get_type()),".")
                    )
                } else {
                    l.push(args[1].to_owned());
                    Ok(Value::List(l.to_owned()))
                }
            } else {
                l.push(args[1].to_owned());
                Ok(Value::List(l.to_owned()))
            }
        } else if let Value::String(mut s) = args[0].clone() {
            s.push_str(&format!("{}", &args[1]));
            Ok(Value::String(s.to_owned()))
        } else if let Value::Object(mut map) = args[0].clone() {
            if let Value::String(s) = &args[1] {
                if args.len() != 3 {
                    if map.contains_key(s) {
                        Err(
                            crate::error!("Key `", s,"` is already present in the targetted object.")
                        )
                    } else {
                        map.insert(s.to_owned(), Value::Nil);
                        Ok(
                            Value::Object(
                                map
                            )
                        )
                    }
                } else {
                    map.insert(s.to_owned(), args[2].to_owned());
                    Ok(
                        Value::Object(
                            map
                        )
                    )
                }
            } else {
                Err(
                    crate::error!("Invalid argument, expected string, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                crate::error!("Invalid argument, expected string, list or object, found", (&args[0].get_type()))
            )
        }
    }
}