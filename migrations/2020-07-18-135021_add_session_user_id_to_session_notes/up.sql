ALTER TABLE session_notes ADD COLUMN session_user_id int NOT NULL;
ALTER TABLE session_notes ADD FOREIGN KEY (session_user_id) REFERENCES session_users(id);
ALTER TABLE session_boards ADD COLUMN session_user_id int NOT NULL;
ALTER TABLE session_boards ADD FOREIGN KEY (session_user_id) REFERENCES session_users(id);
