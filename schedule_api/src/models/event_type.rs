use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Can't match {0} to an event type")]
    UnknownEventType(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Lecture,
    Practical,
    Laboratory,
    Consultation,
    FinalTest, // Залік
    Exam,
    CourseWork,
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EventType::Lecture => "Лк",
            EventType::Practical => "Пз",
            EventType::Laboratory => "Лб",
            EventType::Consultation => "Конс",
            EventType::FinalTest => "Зал",
            EventType::Exam => "Екз",
            EventType::CourseWork => "КП/КР",
        };

        write!(f, "{s}")
    }
}

impl TryFrom<i32> for EventType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value / 10 {
            0 => Ok(EventType::Lecture),
            1 => Ok(EventType::Practical),
            2 => Ok(EventType::Laboratory),
            3 => Ok(EventType::Consultation),
            4 => Ok(EventType::FinalTest),
            5 => Ok(EventType::Exam),
            6 => Ok(EventType::CourseWork),
            _ => Err(Error::UnknownEventType(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_from_int() {
        assert_eq!(EventType::try_from(1).unwrap(), EventType::Lecture);
        assert_eq!(EventType::try_from(10).unwrap(), EventType::Practical);
        assert_eq!(EventType::try_from(20).unwrap(), EventType::Laboratory);
        assert_eq!(EventType::try_from(30).unwrap(), EventType::Consultation);
        assert_eq!(EventType::try_from(40).unwrap(), EventType::FinalTest);
        assert_eq!(EventType::try_from(50).unwrap(), EventType::Exam);
        assert_eq!(EventType::try_from(60).unwrap(), EventType::CourseWork);

        assert!(EventType::try_from(70).is_err());
    }
}
