CREATE TABLE IF NOT EXISTS  users (
	id varchar(100) NOT NULL,
    full_name varchar(255) NOT NULL,
	email varchar(255) NOT NULL UNIQUE,
    blocked tinyint(1) NOT NULL DEFAULT '0',
    user_type varchar(10) NOT NULL,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS  coaches (
	id varchar(100) NOT NULL,
    user_id varchar(100) NOT NULL,
    full_name varchar(255) NOT NULL,
	email varchar(255) NOT NULL UNIQUE,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  programs (
	id varchar(100) NOT NULL,
    name varchar(255) NOT NULL,
    description text, 
    active tinyint(1) NOT NULL DEFAULT '1',
    coach_name varchar(255) NOT NULL,
    coach_id varchar(100) NOT NULL,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (coach_id) REFERENCES coaches(id)
);

CREATE TABLE IF NOT EXISTS  enrollments (
	id varchar(100) NOT NULL,
    program_id varchar(100) NOT NULL,
    member_id varchar(100) NOT NULL,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (program_id) REFERENCES programs(id),
    FOREIGN KEY (member_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS  sessions (
	id varchar(100) NOT NULL,
    name varchar(255) NOT NULL,
    description text,
    program_id varchar(100) NOT NULL,
	enrollment_id varchar(100) NOT NULL,
    people text,
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
    cancelled_at datetime,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (program_id) REFERENCES programs(id),
	FOREIGN KEY (enrollment_id) REFERENCES enrollments(id)
);

CREATE TABLE IF NOT EXISTS  session_users (
    id varchar(100) NOT NULL,
    session_id varchar(100) NOT NULL,
    user_id varchar(100) NOT NULL,
    user_type varchar(10) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS  session_notes (
	id varchar(100) NOT NULL,
    session_id varchar(100) NOT NULL,
    created_by_id varchar(100) NOT NULL,
    session_user_id varchar(100) NOT NULL,
    description text NOT NULL,
    remind_at datetime,
    is_private tinyint(1) NOT NULL DEFAULT '0',
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_id) REFERENCES sessions(id),
    FOREIGN KEY (created_by_id) REFERENCES users(id),
    FOREIGN KEY (session_user_id) REFERENCES session_users(id)
);

CREATE TABLE IF NOT EXISTS  session_files (
	id varchar(100) NOT NULL,
    session_note_id varchar(100) NOT NULL,
    file_name varchar(255) NOT NULL,
    file_path varchar(255) NOT NULL,
    file_type varchar(255),
    file_size int,
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (session_note_id) REFERENCES session_notes(id)
);

CREATE TABLE IF NOT EXISTS  objectives (
	id varchar(100) NOT NULL,
	enrollment_id varchar(100) NOT NULL,
	duration int NOT NULL DEFAULT 1,
   	original_start_date datetime NOT NULL,
  	original_end_date datetime NOT NULL,
 	revised_start_date datetime,
  	revised_end_date datetime,
	actual_start_date datetime,
	actual_end_date datetime,  
	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (enrollment_id) REFERENCES enrollments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  tasks (
    id varchar(100) NOT NULL,
	enrollment_id varchar(100) NOT NULL,	
    actor_id varchar(100) NOT NULL,	
	name varchar(255) NOT NULL,
	duration int NOT NULL DEFAULT 1,
	min int NOT NULL DEFAULT 1,
	max int NOT NULL DEFAULT 1,
   	original_start_date datetime NOT NULL,
  	original_end_date datetime NOT NULL,
 	revised_start_date datetime,
  	revised_end_date datetime,
	offered_start_date datetime,  
	offered_end_date datetime,
	actual_start_date datetime,
	actual_end_date datetime,  
	locked tinyint(1) NOT NULL DEFAULT '0',
	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (enrollment_id) REFERENCES enrollments(id) ON DELETE CASCADE,
	FOREIGN KEY (actor_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS  task_links (
	id varchar(100) NOT NULL,
	source_task_id varchar(100) NOT NULL,
	target_task_id varchar(100) NOT NULL,
	lead_time int NOT NULL DEFAULT 0,
	buffer_time int NOT NULL DEFAULT 0,
	coordinates text NOT NULL,
	priority int NOT NULL DEFAULT 1,
	is_forward tinyint(1) NOT NULL DEFAULT '1',
	PRIMARY KEY (id),
	FOREIGN KEY (source_task_id) REFERENCES tasks(id) ON DELETE CASCADE,
	FOREIGN KEY (target_task_id) REFERENCES tasks(id) ON DELETE CASCADE 
);