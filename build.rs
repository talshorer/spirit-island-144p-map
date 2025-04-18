use std::{env, fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct Islet {
    name: String,
    emoji: String,
    bitcrafter: Option<String>,
}

#[derive(Deserialize)]
struct Islets(Vec<Islet>);

impl Islets {
    fn iter(&self) -> impl Iterator<Item = &Islet> {
        self.0.iter()
    }
}

fn to_pascal_ident(islet: &Islet) -> syn::Ident {
    let mut name = islet.name.clone();
    let mut first = String::new();
    first.push(name.remove(0).to_ascii_uppercase());
    quote::format_ident!("{first}{name}")
}

fn main() {
    const INPUT: &str = "islets.json5";
    println!("cargo::rerun-if-changed={INPUT}");

    let islets: Islets =
        json5::from_str(&String::from_utf8(fs::read(INPUT).unwrap()).unwrap()).unwrap();
    let variants: Vec<_> = islets.iter().map(to_pascal_ident).collect();

    let emoji_cases = islets.iter().zip(&variants).map(|(islet, ident)| {
        let emoji = islet.emoji.as_str();
        quote::quote! {Islet::#ident => #emoji}
    });

    let screenshot_cases = islets.iter().zip(&variants).map(|(islet, ident)| {
        let screenshot_method = match &islet.bitcrafter {
            Some(uuid) => quote::quote! {ScreenshotMethod::Bitcrafter(#uuid)},
            None => quote::quote! {ScreenshotMethod::AbandonedIslet},
        };
        quote::quote! {Islet::#ident => #screenshot_method}
    });

    let syntax_tree = quote::quote! {
        #[derive(strum::Display, strum::EnumIter, Clone, Copy, PartialEq, Eq)]
        enum Islet {
            #(#variants,)*
        }

        impl Islet {
            const fn emoji(&self) -> &str {
                match self {
                    #(#emoji_cases,)*
                }
            }

            const fn screenshot_method(&self) -> ScreenshotMethod {
                match self {
                    #(#screenshot_cases,)*
                }
            }
        }
    };

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let payload = prettyplease::unparse(&syn::parse2(syntax_tree).unwrap());
    let out_path = Path::new(&out_dir).join("islets.rs");
    fs::write(out_path, payload).unwrap();
}
