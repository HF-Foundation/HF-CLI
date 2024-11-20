use hf_codegen::target::Target;
use hf_parser_rust::{ast::SyntaxError, token::TokenizerError};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("io error: {0}")]
    IoError(std::io::Error),

    #[error("error during tokenization: {0:?}")]
    TokenizerError(TokenizerError),

    #[error("error while building ast: {0:?}")]
    AstBuilderError(SyntaxError),
}

impl CompilationError {
    pub fn pretty_print(&self, path: &Path, code: &str) {
        // TODO: Handle this case nicer
        if let Self::IoError(e) = self {
            eprintln!("IO error: {}", e);
            return;
        }

        let location = match self {
            CompilationError::TokenizerError(e) => e.location,
            CompilationError::AstBuilderError(e) => e.location,
            _ => unimplemented!(),
        };
        let span_offset = match self {
            CompilationError::TokenizerError(e) => (0, 1),
            CompilationError::AstBuilderError(e) => e.span(),
            _ => unimplemented!(),
        };

        let err_fmt = match self {
            CompilationError::TokenizerError(e) => format!("{:?}", e),
            CompilationError::AstBuilderError(e) => format!("{:?}", e),
            _ => unimplemented!(),
        };

        let lines = code.lines().collect::<Vec<_>>();

        let underline_line = location.0 + span_offset.0;
        // TODO: If we encounter a new line (span_offset.0 > 0) we
        //       should count the longest line within our span
        let underline_len = if span_offset.0 == 0 {
            location.1 + span_offset.1
        } else {
            span_offset.1
        };

        let line_min = location.0.saturating_sub(2);
        let line_max = underline_line.saturating_add(3).min(lines.len());
        let relevant_lines = lines
            .iter()
            .enumerate()
            .skip(line_min)
            .take(line_max - line_min)
            .map(|(i, s)| (i, s.to_string()))
            .collect::<Vec<_>>();

        eprintln!("error: {}", err_fmt);
        eprintln!("-> {}:{}:{}", path.display(), location.0 + 1, location.1 + 1);
        for (i, line) in relevant_lines {
            eprintln!("{:4} | {}", i + 1, line,);
            if i == underline_line {
                let underline = (0..location.1)
                    .map(|_| ' ')
                    .chain("^".repeat(underline_len).chars())
                    .collect::<String>();
                eprintln!("     | {}", underline);
            }
        }
    }
}

pub struct CompileSettings {
    pub optimization: u8,
}

pub fn compile(
    path: PathBuf,
    target: Target,
    settings: &CompileSettings,
) -> Result<(), CompilationError> {
    // let code = std::fs::read_to_string(path).map_err(|e| CompilationError::IoError(e))?;
    // let tokens =
    //     hf_parser_rust::token::tokenize(&code).map_err(|e| CompilationError::TokenizerError(e))?;
    // println!("Tokens:\n{:#?}\n", tokens);
    // let ast =
    //     hf_parser_rust::ast::build_ast(tokens).map_err(|e| CompilationError::AstBuilderError(e))?;
    // println!("Ast:\n{:#?}\n", ast);
    // // let ir = hf_codegen::ir::from_ast(ast);
    // todo!()

    let code = std::fs::read_to_string(&path).map_err(|e| CompilationError::IoError(e))?;
    let tokens = match hf_parser_rust::token::tokenize(&code) {
        Ok(tokens) => {
            println!("Tokens:\n{:#?}\n", tokens);
            tokens
        }
        Err(e) => {
            let e = CompilationError::TokenizerError(e);
            e.pretty_print(path.as_path(), &code);
            return Err(e);
        }
    };

    let ast = match hf_parser_rust::ast::build_ast(tokens) {
        Ok(ast) => {
            println!("Ast:\n{:#?}\n", ast);
            ast
        }
        Err(e) => {
            let e = CompilationError::AstBuilderError(e);
            e.pretty_print(path.as_path(), &code);
            return Err(e);
        }
    };

    todo!()
}
