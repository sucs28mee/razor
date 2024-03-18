use crate::{
    parser::item::{Item, ItemKind, Ty},
    util::{Span, Spanned},
};

#[derive(Debug, Clone, Copy)]
pub enum CodegenError {
    IncorrectMain,
}

/// Generates `C` code out of `Razor` items.
pub fn gen_c(
    items: impl IntoIterator<Item = Spanned<Item>>,
) -> Result<String, Spanned<CodegenError>> {
    let mut code = String::new();
    for item in items {
        code += "\n";
        match item.value {
            Item {
                ident,
                kind: ItemKind::Fn { args, ty, block: _ },
            } => {
                if *ident == "main" {
                    if !args.is_empty() {
                        return Err(CodegenError::IncorrectMain.span(ident.start..ident.end));
                    }

                    code += "int main() { return 0; }";
                    continue;
                }

                let c_args = args
                    .into_iter()
                    .map(|arg| c_type(Some(arg.ty)) + " " + &arg.ident.value)
                    .collect::<Vec<_>>()
                    .join(", ");

                code += &format!("{} {}({}) {{\n}}", c_type(ty), ident.value, c_args);
            }
        }
    }

    Ok(code)
}

fn c_type(ty: Option<Ty>) -> String {
    let Some(ty) = ty else {
        return "void".to_owned();
    };

    if ty.optional {
        format!("optional_{}", ty.ident.value)
    } else {
        ty.ident.value
    }
}
