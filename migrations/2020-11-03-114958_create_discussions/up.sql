CREATE TABLE IF NOT EXISTS discussions (
    id varchar(50) NOT NULL,
    enrollment_id varchar(100) NOT NULL,
    created_by_id varchar(100) NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (id), 
    FOREIGN KEY (enrollment_id) REFERENCES enrollments(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);