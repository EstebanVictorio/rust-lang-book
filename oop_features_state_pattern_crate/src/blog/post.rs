use super::state::State;

pub struct Post {
    state: State,
    content: String,
    revision: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: State::Draft,
            content: String::new(),
            revision: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.revision.push_str(text);

        self.transition(Some(State::Draft));
    }

    pub fn approve(&mut self) {
        self.transition(None);
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn transition(&mut self, force: Option<State>) {
        if let None = force {
            self.state = match self.state {
                State::Draft => State::Review,
                State::Published => State::Published,
                State::Review => {
                    self.content = self.revision.clone();
                    State::Published
                }
            };
            return;
        }

        // if tuple conditions can be used for multiple values, but ownership rules also apply here, so, we use references
        if let (State::Review, Some(State::Published)) = (&self.state, &force) {
            self.content = self.revision.clone();
        }

        self.state = force.unwrap();
    }
}
