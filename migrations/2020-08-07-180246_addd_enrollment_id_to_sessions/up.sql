ALTER TABLE sessions ADD COLUMN enrollment_id int NOT NULL;
ALTER TABLE sessions ADD FOREIGN KEY (enrollment_id) REFERENCES enrollments(id);