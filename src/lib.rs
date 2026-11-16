// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-login-manager-backend: login manager backend component model.
//!
//! Real implementation: backends, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backend {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginManagerBackendError {
    EmptyBackendName,
    DuplicateBackend(String),
    EmptyEventSubject,
}

impl fmt::Display for LoginManagerBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBackendName => write!(f, "backend name must not be empty"),
            Self::DuplicateBackend(id) => write!(f, "duplicate backend {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LoginManagerBackendError {}

pub struct LoginManagerBackend {
    pub name: String,
    backends: BTreeMap<String, Backend>,
    events: BTreeMap<String, Event>,
}

impl LoginManagerBackend {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            backends: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_backend(&mut self, item: Backend) -> Result<(), LoginManagerBackendError> {
        if item.name.trim().is_empty() {
            return Err(LoginManagerBackendError::EmptyBackendName);
        }
        let id = item.id.clone();
        if self.backends.contains_key(&id) {
            return Err(LoginManagerBackendError::DuplicateBackend(id.clone()));
        }
        self.backends.insert(id, item);
        Ok(())
    }

    pub fn backend_count(&self) -> usize {
        self.backends.len()
    }

    pub fn get_backend(&self, id: &str) -> Option<&Backend> {
        self.backends.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LoginManagerBackendError> {
        if event.subject.trim().is_empty() {
            return Err(LoginManagerBackendError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_backend(id: &str, name: &str) -> Backend {
        Backend {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LoginManagerBackend::new("LoginManagerBackend");
        assert_eq!(m.name, "LoginManagerBackend");
        assert_eq!(m.backend_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_backend() {
        let mut m = LoginManagerBackend::new("LoginManagerBackend");
        m.add_backend(sample_backend("a1", "Alpha")).unwrap();
        assert_eq!(m.backend_count(), 1);
        assert!(m.get_backend("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LoginManagerBackend::new("LoginManagerBackend");
        assert!(m.add_backend(sample_backend("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LoginManagerBackend::new("LoginManagerBackend");
        m.add_backend(sample_backend("x", "One")).unwrap();
        assert!(m.add_backend(sample_backend("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LoginManagerBackend::new("LoginManagerBackend");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
