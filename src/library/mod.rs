mod consts;
mod helper;

use std::{
    collections::{BTreeMap, HashMap},
    fmt::{self, Display},
    io::Write,
    process::{self},
};

#[derive(Debug)]
pub enum ErrMessage {
    Syntax(String),
    OpCode(String),
    Other(String),
}

#[derive(Debug)]
enum Operator {
    Find,
    Replace,
    Uppercase,
    Lowercase,
    Delete,
    NewLine,
    Err,
}
pub struct Maggedik {
    operator: Operator,
    arguments: Vec<String>,
}
#[derive(Debug)]
pub struct FindOutput {
    op_type: Operator,
    total: usize,
    contents: Vec<String>,
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl Display for ErrMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error : {:?}", self)
    }
}

impl Display for FindOutput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "total matches : {}", self.total)?;
        if self.total > 0 {
            for (count, content) in self.contents.iter().enumerate() {
                if count != 0 {
                    writeln!(formatter, "{}", content)?;
                } else {
                    writeln!(formatter, "{}", content)?;
                }
            }
        } else {
            writeln!(formatter, "no result found..")?;
        }
        writeln!(formatter, "Result type : {}", self.op_type)
    }
}

impl Maggedik {
    pub fn new(arguments: Vec<String>) -> Result<Maggedik, ErrMessage> {
        if arguments.len() < 3 {
            return Err(ErrMessage::Syntax("invalid syntax..".to_owned()));
        }
        let op = set_operator(&arguments[consts::TWO]);

        if let Operator::Err = op {
            return Err(ErrMessage::OpCode(
                "invalid operation 
            code.."
                    .to_owned(),
            ));
        }

        if !&arguments[consts::ONE]
            .split(".")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap_or_else(|| {
                eprintln!("unable to fetch file extenssion..");
                process::exit(0);
            })
            .eq(&consts::TXT)
        {
            return Err(ErrMessage::Other(
                "unsupported file extenssion..".to_owned(),
            ));
        }
        Ok(Maggedik {
            operator: op,
            arguments,
        })
    }

    pub fn process(&self) -> Result<FindOutput, ErrMessage> {
        let result = match self.operator {
            Operator::Find => self.find(),
            Operator::Replace => self.replace(),
            Operator::Uppercase => self.uppercase(),
            Operator::Lowercase => self.lowercase(),
            Operator::Delete => self.delete(),
            Operator::NewLine => self.new_line(),
            Operator::Err => return Err(ErrMessage::OpCode("invalid operation code..".to_owned())),
        };
        Ok(result)
    }

    fn find(&self) -> FindOutput {
        do_action(
            Operator::Find,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::None,
        )
    }
    fn replace(&self) -> FindOutput {
        do_action(
            Operator::Replace,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::Some(&self.arguments[consts::FOUR]),
        )
    }

    fn uppercase(&self) -> FindOutput {
        do_action(
            Operator::Uppercase,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::None,
        )
    }

    fn lowercase(&self) -> FindOutput {
        do_action(
            Operator::Lowercase,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::None,
        )
    }

    fn delete(&self) -> FindOutput {
        do_action(
            Operator::Delete,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::None,
        )
    }

    fn new_line(&self) -> FindOutput {
        do_action(
            Operator::NewLine,
            &self.arguments[consts::ONE],
            &self.arguments[consts::THREE],
            Option::None,
        )
    }
}

fn do_action(
    operator: Operator,
    file_dest: &String,
    keyword: &String,
    to_keyword: Option<&str>,
) -> FindOutput {
    let lines = helper::get_lines(file_dest);
    let mut find_indexes = Vec::<String>::new();
    match operator {
        Operator::Find => {
            let total = lines
                .enumerate()
                .filter(|(index, line)| {
                    if line
                        .as_ref()
                        .unwrap_or_else(|err| {
                            eprintln!("{}", err.to_string());
                            process::exit(0);
                        })
                        .contains(keyword)
                    {
                        find_indexes.push(format!("at line : {}", index + 1));
                        true
                    } else {
                        false
                    }
                })
                .count();
            FindOutput {
                op_type: Operator::Find,
                total,
                contents: find_indexes,
            }
        }
        _ => {
            let mut write_lines = String::new();
            let mut lines_words = Box::new(BTreeMap::<usize, String>::new());
            let changed_lines = lines
                .enumerate()
                .filter(|(index, line)| {
                    let line = line.as_ref().unwrap_or_else(|err| {
                        eprintln!("{}", err.to_string());
                        process::exit(0);
                    });
                    lines_words.insert(*index, line.to_owned());
                    if line.contains(keyword) {
                        find_indexes.push(format!("at line : {}", index + 1));
                        true
                    } else {
                        false
                    }
                })
                .map(|(index, line)| {
                    let linecpy = line.unwrap_or_else(|err| {
                        eprintln!("{}", err.to_string());
                        process::exit(0);
                    });
                    let line: String = linecpy
                        .split_ascii_whitespace()
                        .into_iter()
                        .filter(|word| keyword.eq(word))
                        .map(|word| match operator {
                            Operator::Replace => linecpy.replace(word, &to_keyword.unwrap()),
                            Operator::Lowercase => {
                                linecpy.replace(word, &keyword.to_ascii_lowercase())
                            }
                            Operator::Uppercase => {
                                linecpy.replace(word, &keyword.to_ascii_uppercase())
                            }
                            Operator::Delete => linecpy.replace(word, ""),
                            Operator::NewLine => linecpy.replace(
                                word,
                                (keyword.to_owned() + "" + consts::NEW_LINE).as_str(),
                            ),
                            _ => String::new(),
                        })
                        .collect();
                    (index, line)
                })
                .collect::<HashMap<usize, String>>();
            for (inner_index, inner_line) in lines_words.into_iter() {
                if changed_lines.contains_key(&inner_index) {
                    write_lines.push_str(changed_lines.get(&inner_index).unwrap());
                } else {
                    write_lines.push_str(&inner_line.as_str());
                }
                write_lines.push_str(consts::NEW_LINE);
            }
            helper::create_or_get_file(file_dest)
                .write_all(write_lines.as_bytes())
                .unwrap_or_else(|err| {
                    eprintln!("{}", err.to_string());
                    process::exit(0);
                });
            FindOutput {
                op_type: operator,
                total: changed_lines.len(),
                contents: find_indexes,
            }
        }
    }
}

fn set_operator(operator: &String) -> Operator {
    let op = match operator.as_str() {
        "f" => Operator::Find,
        "rp" => Operator::Replace,
        "uc" => Operator::Uppercase,
        "lc" => Operator::Lowercase,
        "d" => Operator::Delete,
        "nl" => Operator::NewLine,
        _ => Operator::Err,
    };
    op
}
