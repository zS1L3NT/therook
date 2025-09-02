use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use regex::Regex;
use syn::parse_macro_input;

#[proc_macro]
pub fn bitboard(input: TokenStream) -> TokenStream {
    let string = input.to_string();

    if string.len() == 2 {
        let mut chars = string.chars();
        let file = chars.next().unwrap();
        let rank = chars.next().unwrap();

        format!("crate::engine::FILE_{file} & crate::engine::RANK_{rank}")
            .parse()
            .unwrap()
    } else {
        let mut chars = string.split_whitespace();
        let mut file: String = chars.next().unwrap().into();
        let mut rank: String = chars.next().unwrap().into();

        if file.len() == 1 {
            file = format!("crate::engine::FILE_{file}");
        }

        if rank.len() == 1 {
            rank = format!("crate::engine::RANK_{rank}");
        }

        format!("{file} & {rank}").parse().unwrap()
    }
}

#[proc_macro]
pub fn tile(input: TokenStream) -> TokenStream {
    format!("Into::<crate::engine::Tile>::into({})", bitboard(input))
        .parse()
        .unwrap()
}

#[proc_macro_attribute]
pub fn timed(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as syn::Ident);
    let itemfn = parse_macro_input!(item as syn::ItemFn);

    let vis = &itemfn.vis;
    let sig = &itemfn.sig;
    let block = &itemfn.block;

    let struct_name = ident.to_string();
    let function_name = sig.ident.to_string();

    let regex = Regex::new(r" ?([&:<>]) ?").unwrap();
    let args = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(receiver) => regex
                .replace_all(&receiver.to_token_stream().to_string(), "$1")
                .into(),
            syn::FnArg::Typed(pat_type) => {
                format!(
                    "{}: {}",
                    pat_type.pat.to_token_stream(),
                    regex.replace_all(&pat_type.ty.to_token_stream().to_string(), "$1")
                )
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    let tokens = quote! {
        #vis #sig {
            let start = std::time::Instant::now();
            let result = { #block };
            println!("{}::{}({}) took {} nanoseconds to execute", #struct_name, #function_name, #args, start.elapsed().as_nanos());
            result
        }
    };

    tokens.into()
}
