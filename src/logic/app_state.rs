use std::sync::{Arc, Mutex};

use actix_web::web;
use uuid::Uuid;

use crate::logic::{
    file_master::FileMaster, hasher::Hasher, user_db::UserDB, user_sessions::UserSessions,
};

pub type AppStateType<'a> = web::Data<Arc<Mutex<AppState<'a>>>>;

#[derive(Clone)]
pub struct AppState<'a> {
    file_master: FileMaster,
    hasher: Hasher<'a>,
    user_db: UserDB,
    user_sessions: UserSessions,
}

impl<'a> AppState<'a> {
    pub fn new() -> std::io::Result<Self> {
        // order is important since FileMaster creates directories that
        // UserDB assumes exist
        let fm = FileMaster::new()?;
        let udb = UserDB::new()?;

        Ok(AppState {
            file_master: fm,
            hasher: Hasher::new(64),
            user_db: udb,
            user_sessions: UserSessions::new(),
        })
    }

    pub fn add_session(&mut self, session_id: &String, username: &String) {
        self.user_sessions.add_session(session_id, username);
    }

    pub fn session_exists(&self, session_id: &String) -> bool {
        self.user_sessions.exists(session_id)
    }

    pub fn get_username_from_session(&self, session_id: &String) -> Option<String> {
        self.user_sessions.get_username(session_id)
    }

    pub fn remove_session(&mut self, session_id: &String) {
        self.user_sessions.remove_session(session_id);
    }

    pub fn get_user_db<S: AsRef<str>>(&self, username: &S) -> Option<String> {
        self.user_db.get_user(username)
    }

    pub fn add_user_db<S: AsRef<str>, T: AsRef<str>>(&mut self, username: &S, password: &T) {
        self.user_db.add_user(username, password)
    }

    pub fn hash<S: AsRef<str>>(&self, password: &S) -> String {
        self.hasher.hash(password)
    }

    pub fn validate_hash<T: AsRef<str>, S: AsRef<str>>(&self, hash: &T, password: &S) -> bool {
        self.hasher.validate(hash, password)
    }

    pub fn unique_string(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn store_paste(&mut self, content: &String, path: &String) -> std::io::Result<String> {
        self.file_master.store_file(content, path)
    }

    pub fn read_paste(&self, name: &String) -> std::io::Result<String> {
        self.file_master.read_file(name)
    }
}
