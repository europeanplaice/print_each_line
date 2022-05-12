use proc_macro::{self, TokenStream};
use proc_macro2;
use proc_macro2::Delimiter;
use proc_macro2::Span;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro_attribute]
pub fn printline(_attr: TokenStream, item_old: TokenStream) -> TokenStream {
    //
    let instant: proc_macro2::TokenStream = quote! {
        use std::time::Instant;
    };
    let nowstream: proc_macro2::TokenStream = quote! {
        let now = Instant::now();
    };
    let time_result: proc_macro2::TokenStream = quote! {
        println!(" ... ⌛ (Elapsed {} milliseconds) ⌛", now.elapsed().as_millis());
    };
    //
    let item = item_old.clone();
    let item = parse_macro_input!(item as proc_macro2::TokenStream);
    let mut _new_stream = proc_macro2::TokenStream::new();
    for item2 in item.clone() {
        let tt = match item2 {
            TokenTree::Group(ref group) => {
                let g = match group.delimiter() {
                    Delimiter::Brace => {
                        //function body
                        let body_stream = group.stream();
                        let mut _new_stream_oneline = proc_macro2::TokenStream::new();
                        _new_stream_oneline.extend(instant.clone());
                        _new_stream_oneline.extend(nowstream.clone());
                        let mut new_literals = vec![];
                        let punct = proc_macro2::Punct::new(';', proc_macro2::Spacing::Alone);
                        let print_ident =
                            TokenTree::Ident(proc_macro2::Ident::new("print", Span::call_site()));
                        let print_punct = TokenTree::Punct(proc_macro2::Punct::new(
                            '!',
                            proc_macro2::Spacing::Joint,
                        ));
                        for tt in body_stream {
                            match tt.clone() {
                                TokenTree::Punct(pun) => {
                                    if pun.as_char() == ';' {
                                        let mut str_stream = proc_macro2::TokenStream::new();
                                        let mut joined = new_literals.join("");
                                        joined = format!("Ran ... 📄 {} 📄", joined);
                                        let lit = proc_macro2::Literal::string(&joined);
                                        str_stream.extend(std::iter::once(
                                            proc_macro2::TokenTree::Literal(lit),
                                        ));
                                        let print_brace =
                                            TokenTree::Group(proc_macro2::Group::new(
                                                proc_macro2::Delimiter::Brace,
                                                str_stream,
                                            ));

                                        // println!(...);
                                        _new_stream_oneline.extend(std::iter::once(
                                            TokenTree::Punct(punct.clone()),
                                        ));
                                        _new_stream_oneline
                                            .extend(std::iter::once(print_ident.clone()));
                                        _new_stream_oneline
                                            .extend(std::iter::once(print_punct.clone()));
                                        _new_stream_oneline.extend(std::iter::once(print_brace));
                                        _new_stream_oneline.extend(std::iter::once(
                                            TokenTree::Punct(punct.clone()),
                                        ));
                                        new_literals = vec![];
                                        _new_stream_oneline.extend(time_result.clone());
                                        _new_stream_oneline.extend(nowstream.clone());
                                    } else {
                                        _new_stream_oneline.extend(std::iter::once(tt.clone()));
                                        new_literals.push(format!("{}", tt));
                                        match pun.spacing() {
                                            proc_macro2::Spacing::Joint => {}
                                            proc_macro2::Spacing::Alone => {
                                                new_literals.push(" ".to_string());
                                            }
                                        }
                                    }
                                }
                                TokenTree::Group(g) => match g.delimiter() {
                                    proc_macro2::Delimiter::Brace => {
                                        _new_stream_oneline.extend(std::iter::once(tt.clone()));

                                        let mut formatted = format!("{}", tt);
                                        formatted = formatted.replace("{", "{{");
                                        formatted = formatted.replace("}", "}}");
                                        new_literals.push(format!("{}", formatted));
                                        new_literals.push(" ".to_string());
                                    }
                                    _ => {
                                        _new_stream_oneline.extend(std::iter::once(tt.clone()));
                                        let mut formatted = format!("{}", tt);
                                        formatted = formatted.replace("{", "{{");
                                        formatted = formatted.replace("}", "}}");
                                        new_literals.push(format!("{}", formatted));
                                        new_literals.push(" ".to_string());
                                    }
                                },
                                TokenTree::Ident(_) => {
                                    _new_stream_oneline.extend(std::iter::once(tt.clone()));
                                    new_literals.push(format!("{}", tt));
                                    new_literals.push(" ".to_string());
                                }
                                TokenTree::Literal(_lit) => {
                                    _new_stream_oneline.extend(std::iter::once(tt.clone()));
                                    let mut formatted = format!("{}", tt);
                                    formatted = formatted.replace("{", "{{");
                                    formatted = formatted.replace("}", "}}");
                                    new_literals.push(format!("{}", formatted));
                                }
                            }
                        }
                        proc_macro2::Group::new(group.delimiter(), _new_stream_oneline)
                    }
                    _ => group.clone(),
                };
                TokenTree::Group(g)
            }
            TokenTree::Ident(ident) => TokenTree::Ident(ident),
            TokenTree::Punct(punct) => TokenTree::Punct(punct),
            TokenTree::Literal(literal) => TokenTree::Literal(literal),
        };
        _new_stream.extend(std::iter::once(tt))
    }

    _new_stream.into()
}
