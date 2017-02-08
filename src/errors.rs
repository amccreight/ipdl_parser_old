use ast::Location;

fn error_msg(loc: &Location, err: &str) -> String {
    format!("{}: error: {}", loc, err)
}

#[must_use]
pub struct Errors {
    errors: Vec<String>
}

impl Errors {
    pub fn none() -> Errors {
        Errors { errors: Vec::new() }
    }

    pub fn one(loc: &Location, err: &str) -> Errors {
        Errors { errors: vec![error_msg(&loc, &err)] }
    }

    pub fn append(&mut self, mut other: Errors) {
        self.errors.append(&mut other.errors);
    }

    pub fn append_one(&mut self, loc: &Location, other: &str) {
        self.errors.push(error_msg(&loc, &other));
    }

    pub fn to_result(&self) -> Result<(), String> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.join("\n"))
        }
    }
}
