CREATE TABLE IF NOT EXISTS  users (
	id int NOT NULL AUTO_INCREMENT,
    full_name varchar(255) NOT NULL,
	email varchar(255) NOT NULL UNIQUE,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    blocked tinyint(1) NOT NULL DEFAULT '0',
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id)
);


CREATE TABLE IF NOT EXISTS  teams (
	id int NOT NULL AUTO_INCREMENT,
    name varchar(255) NOT NULL,
	fuzzy_id varchar(255) NOT NULL UNIQUE,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS  team_members (
	id int NOT NULL AUTO_INCREMENT,
    team_id int NOT NULL,
    user_id int NOT NULL,
    blocked tinyint(1) NOT NULL DEFAULT '0',
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  programs (
	id int NOT NULL AUTO_INCREMENT,
	name varchar(255) NOT NULL,
    coach_id int NOT NULL,
  	active tinyint(1) NOT NULL DEFAULT '1',
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (coach_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  sessions (
	id int NOT NULL AUTO_INCREMENT,
	program_id int NOT NULL,
	name varchar(255) NOT NULL,
	duration int NOT NULL DEFAULT 1,
   	original_start_date datetime NOT NULL,
  	original_end_date datetime NOT NULL,
 	revised_start_date datetime,
  	revised_end_date datetime,
	offered_start_date datetime,  
	offered_end_date datetime,
	is_ready tinyint(1) NOT NULL DEFAULT '0',
	actual_start_date datetime,
	actual_end_date datetime,  
	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (program_id) REFERENCES programs(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  session_links (
	id int NOT NULL AUTO_INCREMENT,
	source_session_id int NOT NULL,
	target_session_id int NOT NULL,
	lead_time int NOT NULL DEFAULT 0,
	buffer_time int NOT NULL DEFAULT 0,
	coordinates text NOT NULL,
	priority int NOT NULL DEFAULT 1,
	is_forward tinyint(1) NOT NULL DEFAULT '1',
	PRIMARY KEY (id),
	FOREIGN KEY (source_session_id) REFERENCES sessions(id) ON DELETE CASCADE,
	FOREIGN KEY (target_session_id) REFERENCES sessions(id) ON DELETE CASCADE 
);

CREATE TABLE IF NOT EXISTS  enrollments (
	id int NOT NULL AUTO_INCREMENT,
	program_id int NOT NULL,
    team_id int NOT NULL,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (program_id) REFERENCES programs(id) ON DELETE CASCADE,
	FOREIGN KEY (team_id) REFERENCES teams(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  session_visits (
	id int NOT NULL AUTO_INCREMENT,
	session_id int NOT NULL,
    user_id int NOT NULL,
	joined_at  datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

