use proc_macro::TokenStream;
        use log::trace;

#[proc_macro_attribute]
  pub fn duration ($function: expr) {
        use logger::trace;
        use std::time::Instant;
        let start = Instant::now();
        $function;
        let duration = start.elapsed();
        trace!("{:?}", duration)
    };

#[proc_macro_attribute]
pub fn trace_vars(_metadata: TokenStream, input: TokenStream) -> TokenStream {
// parsing rust function to easy to use struct
    let input_fn = parse_macro_input!(input as ItemFn);
    TokenStream::from(quote!{fn dummy(){}})
}

pub fn pipeline_duration (attr: TokenStream, item: TokenStream) -> TokenStream {
        use std::time::Instant;
        let start = Instant::now();
        $function;
        let duration = start.elapsed();
        trace!("{:?}", duration);
}
