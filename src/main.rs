use nakai_rust::{
    attrs::{self, attr, class, lang},
    elements::{self, body, div, head, html, li, ol, p, title, Node},
    render::{render_document, render_inline},
};

pub fn main() {
    // let big_element = (0..1_000_000)
    //     .into_iter()
    //     .map(|_| p([], ["Hello"]))
    //     .collect::<Vec<_>>();
    // let fragment = Node::Fragment {
    //     children: big_element,
    // };
    // let t0 = std::time::Instant::now();
    // render_inline(fragment);
    // println!("{}", t0.elapsed().as_millis());

    let app = html(
        [lang("en-US")],
        [
            head([title("Hello Title")]),
            body(
                [class("body")],
                [div([], [div([], ["hello world".into()])])],
            ),
        ],
    );
    println!("{}", render_inline(app))
}
