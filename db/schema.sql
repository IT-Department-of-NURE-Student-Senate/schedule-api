DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE TYPE EventType AS ENUM ('Лк', 'Пз', 'Лб', 'Конс', 'Зал', 'Екз', 'КП/КР');

CREATE TABLE Faculty (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL
);

CREATE TABLE Department (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL,
    faculty_id INT NOT NULL REFERENCES Faculty (id) ON DELETE CASCADE
);

CREATE TABLE Teacher (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL,
    department_id INT NOT NULL REFERENCES Department (id) ON DELETE CASCADE
);

CREATE TABLE Direction (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL,
    faculty_id INT NOT NULL REFERENCES Faculty (id) ON DELETE CASCADE
);

CREATE TABLE Speciality (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL,
    direction_id INT NOT NULL REFERENCES Direction (id) ON DELETE CASCADE
);

CREATE TABLE AcademicGroup (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    direction_id INT NOT NULL REFERENCES Direction (id) ON DELETE CASCADE,
    speciality_id INT NULL REFERENCES Speciality (id) ON DELETE SET NULL
);

CREATE TABLE Building (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL
);

CREATE TABLE Auditory (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    floor SMALLINT NOT NULL,
    have_power BOOLEAN NOT NULL,
    building_id INT NOT NULL REFERENCES Building (id) ON DELETE CASCADE
);

CREATE TABLE AuditoryType (
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE AuditoryTypeToAuditory (
    auditory_id INT NOT NULL REFERENCES Auditory (id) ON DELETE CASCADE,
    auditory_type_id INT NOT NULL REFERENCES AuditoryType (id) ON DELETE CASCADE,
    PRIMARY KEY (auditory_id, auditory_type_id)
);

CREATE TABLE Subject (
    id INT PRIMARY KEY NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    short_name VARCHAR(40) NOT NULL
);

CREATE TABLE SubjectToTeacher (
    id INT PRIMARY KEY NOT NULL,
    subject_id INT NOT NULL REFERENCES Subject (id) ON DELETE CASCADE,
    teacher_id INT NOT NULL REFERENCES Teacher (id) ON DELETE CASCADE,
    event_type EventType NOT NULL,
    hours SMALLINT NOT NULL
);

CREATE TABLE Event (
    id INT PRIMARY KEY NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    number_pair SMALLINT NOT NULL,
    event_type EventType NOT NULL,
    auditory_id INT NOT NULL REFERENCES Auditory (id) ON DELETE SET NULL,
    subject_id INT NOT NULL REFERENCES Subject (id) ON DELETE CASCADE,
    CHECK (start_time < end_time)
);

CREATE TABLE EventToTeacher (
    event_id INT NOT NULL REFERENCES Event (id) ON DELETE CASCADE,
    teacher_id INT NOT NULL REFERENCES Teacher (id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, teacher_id)
);

CREATE TABLE EventToAcademicGroup (
    event_id INT NOT NULL REFERENCES Event (id) ON DELETE CASCADE,
    group_id INT NOT NULL REFERENCES AcademicGroup (id) ON DELETE CASCADE,
    PRIMARY KEY (event_id, group_id)
);

CREATE TABLE UserSubjectURL (
    id INT PRIMARY KEY NOT NULL,
    url VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    subject_id INT NOT NULL REFERENCES Subject (id) ON DELETE CASCADE
);
