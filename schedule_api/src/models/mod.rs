#![allow(unused_imports)]

mod auditory;
mod auditory_type;
mod building;
mod department;
mod direction;
mod event;
mod event_type;
mod faculty;
mod group;
mod root;
mod speciality;
mod subject;
mod teacher;
mod university;

pub use auditory::Auditory;
pub use auditory_type::AuditoryType;
pub use building::Building;
pub use department::Department;
pub use direction::Direction;
pub use event::Event;
pub use event_type::EventType;
pub use faculty::Faculty;
pub use group::Group;
pub use root::Root;
pub use speciality::Speciality;
pub use subject::Subject;
pub use teacher::Teacher;
pub use university::University;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    EventTypeError(#[from] event_type::Error),
}
