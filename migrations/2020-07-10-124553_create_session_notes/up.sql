CREATE TABLE IF NOT EXISTS  session_notes (
	id int NOT NULL AUTO_INCREMENT,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    session_id int NOT NULL,
    description text NOT NULL,
    remind_at datetime,
    created_by_id int NOT NULL,
    is_private tinyint(1) NOT NULL DEFAULT '0',
 	  created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS  session_files (
	id int NOT NULL AUTO_INCREMENT,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    session_note_id int NOT NULL,
    file_name varchar(255) NOT NULL,
    file_path varchar(255) NOT NULL,
    file_type varchar(255),
    file_size int,
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_note_id) REFERENCES session_notes(id)
);

CREATE TABLE IF NOT EXISTS  session_boards (
	id int NOT NULL AUTO_INCREMENT,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    session_id int NOT NULL,
    file_name varchar(255) NOT NULL,
    file_path varchar(255) NOT NULL,
    created_by_id int NOT NULL,
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id)
);


