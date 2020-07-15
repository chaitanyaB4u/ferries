CREATE TABLE IF NOT EXISTS session_users (
    id int NOT NULL AUTO_INCREMENT,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    session_id int NOT NULL,
    user_id int NOT NULL,
    user_type varchar(255) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
)