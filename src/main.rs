use nakai_rust::{
    html::{self, li, ol, p},
    render::{render_document, render_inline},
};

pub fn main() {
    let big_element = (0..100000)
        .into_iter()
        .map(|_| p([], ["Hello"]))
        .collect::<Vec<_>>();
    let fragment = html::Node::Fragment {
        children: big_element,
    };
    let t0 = std::time::Instant::now();
    render_inline(fragment);
    println!("{}", t0.elapsed().as_millis());
}
