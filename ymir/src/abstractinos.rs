/// A connection between a client + server, client + client, or server + server
pub struct Connection {
    pub connection: &'static str, //TODO: figure this out
}

/// Authority types. This denotes WHO owns a particular piece of state.
pub enum Authority {
    Client,
    Server { connection: Connection },
}

pub enum Persistence {
    Component,
    Server,
    LocalStorage,
    SessionStorage,
}

pub struct State<Value> {
    pub authority: Authority,
    pub persistence: Persistence,
    pub value: Option<Value>,
}

impl<Value> State<Value> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn sync(&mut self) {
        todo!()
    }
}

pub fn run(mut application: Application) {
    loop {
        application.tick();
    }
}

pub struct Container<Item> {
    elements: Vec<Item>,
}

impl<Item> Container<Item> {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }
}

pub enum ViewElement {
    Paragraph(String),
}

pub struct Component {
    pub children: Container<Component>, // TODO: determine whether a tree is necessary
    pub view: Option<ViewElement>,
}

pub struct Application {
    pub view: Component,
    pub update: (),
}

impl Application {
    fn tick(&mut self) {
        println!("A tick!");
    }
}
