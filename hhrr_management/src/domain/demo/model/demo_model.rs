pub struct DemoModel {
    id: String,
    title: String,
    body: String,
    is_published: bool,
}

impl DemoModel {
    pub fn new(title: String, body: String) -> DemoModel {
        DemoModel {
            id: String::from("uuid-generated-string"),
            title,
            body,
            is_published: false,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_body(&self) -> &String {
        &self.body
    }

    pub fn is_published(&self) -> bool {
        self.is_published.clone()
    }
}