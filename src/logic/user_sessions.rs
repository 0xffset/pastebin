use std::collections::HashMap;

#[derive(Clone)]
pub struct UserSessions {
	// <session id, username>
	pub sessions: HashMap<String, String>,
}

impl UserSessions {
	pub fn new() -> UserSessions {
		UserSessions {
			sessions: HashMap::new(),
		}
	}

	pub fn add_session(&mut self, session_id: &String, username: &String) {
		self.sessions.insert(session_id.clone(), username.clone());
	}

	pub fn exists(&self, session_id: &String) -> bool {
		self.sessions.get(session_id).is_some()
	}

	pub fn get_username(&self, session_id: &String) -> Option<String> {
		match self.sessions.get(session_id) {
			Some(username) => Some(username.clone()),
			None => None,
		}
	}

	pub fn remove_session(&mut self, session_id: &String) {
		self.sessions.remove(session_id);
	}
}
