use super::state::State;

pub struct Post {
    state: State,
    content: String,
    revision: String,
    approvals: u32,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: State::Draft,
            content: String::new(),
            revision: String::new(),
            approvals: 0,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn approve(&mut self) {
        self.approvals += 1;
        if self.approvals > 1 {
            self.transition(Some(State::Published));
        }
    }

    pub fn add_text(&mut self, text: &str) {
        let is_draft = match self.state {
            State::Draft => true,
            _ => false,
        };

        if !is_draft {
            return;
        }

        self.revision.push_str(text);
    }

    fn reject(&mut self) {
        self.transition(Some(State::Draft));
    }

    pub fn send_for_review(&mut self) {
        self.transition(Some(State::Review));

        if false {
            self.reject();
        }
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
