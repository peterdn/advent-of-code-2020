#![feature(proc_macro_span)]

extern crate proc_macro;
use proc_macro::{Span, TokenStream};

#[proc_macro]
pub fn aoc_main(token: TokenStream) -> TokenStream {
    let source_stem = Span::call_site().source_file().path();
    let day = source_stem.as_path().file_stem().unwrap().to_str().unwrap();
    format!(
        r#"
        fn main() {{
            let input = aoc::load_input("inputs/{day}");
            let input = input.trim();

            println!("***** Part 1 *****");
            part1({f}(input));

            println!();
            println!("***** Part 2 *****");
            part2({f}(input));
        }}
        "#,
        day = day,
        f = if token.is_empty() {
            "".to_string()
        } else {
            format!("&{}", token)
        }
    )
    .parse()
    .unwrap()
}
