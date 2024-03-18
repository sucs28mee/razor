use crate::{
    parser::item::{Item, ItemKind},
    util::Spanned,
};

pub fn gen_c(items: impl IntoIterator<Item = Spanned<Item>>) -> String {
    let mut code = String::new();
    for item in items {
        match item.value {
            Item {
                ident,
                kind: ItemKind::Fn { args, ty, block: _ },
            } => {
                let c_type = match &ty {
                    Some(ty) => &ty.value,
                    None => "void",
                };

                let c_args = args
                    .into_iter()
                    .map(|arg| arg.ty.value + " " + &arg.ident.value)
                    .collect::<Vec<_>>()
                    .join(", ");

                code += &format!(
                    r#"
                    {} {}({}) {{

                    }}
                "#,
                    c_type, ident.value, c_args
                );
            }
        }
    }

    code
}
