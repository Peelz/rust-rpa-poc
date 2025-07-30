pub struct PublisherConfig<'a> {
    gcp_project_id: &'a str,
    topic_name: &'a str,
}

pub struct Publisher {}

impl Publisher {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Publisher {
    fn default() -> Self {
        Self::new()
    }
}
