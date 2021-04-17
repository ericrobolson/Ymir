pub fn main() {
    let application = Application {
        view: Component {
            children: Container::new(),
            view: Some(ViewElement::Paragraph("Hello World".into())),
        },
        update: (),
    };
    ymir::run(application);
    println!("HELLO WRLD!");
}
