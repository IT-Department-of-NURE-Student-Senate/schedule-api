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
        assert_eq!(EventType::try_from(0).unwrap(), EventType::Lecture);
        assert_eq!(EventType::try_from(1).unwrap(), EventType::Practical);
        assert_eq!(EventType::try_from(2).unwrap(), EventType::Laboratory);
        assert_eq!(EventType::try_from(3).unwrap(), EventType::Consultation);
        assert_eq!(EventType::try_from(4).unwrap(), EventType::FinalTest);
        assert_eq!(EventType::try_from(5).unwrap(), EventType::Exam);
        assert_eq!(EventType::try_from(6).unwrap(), EventType::CourseWork);

        assert!(EventType::try_from(7).is_err());
    }
}
