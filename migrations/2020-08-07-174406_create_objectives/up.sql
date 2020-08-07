CREATE TABLE IF NOT EXISTS  objectives (
	id int NOT NULL AUTO_INCREMENT,
	enrollment_id int NOT NULL,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
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

CREATE TABLE IF NOT EXISTS  observations (
	id int NOT NULL AUTO_INCREMENT,
	enrollment_id int NOT NULL,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
	FOREIGN KEY (enrollment_id) REFERENCES enrollments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  options (
	id int NOT NULL AUTO_INCREMENT,
	enrollment_id int NOT NULL,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
	FOREIGN KEY (enrollment_id) REFERENCES enrollments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS  tasks (
	id int NOT NULL AUTO_INCREMENT,
	enrollment_id int NOT NULL,
	fuzzy_id varchar(255) NOT NULL UNIQUE,
	name varchar(255) NOT NULL,
	duration int NOT NULL DEFAULT 1,
	min int NOT NULL DEFAULT 1,
	max int NOT NULL DEFAULT 1,
	actor_id int NOT NULL,
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
	id int NOT NULL AUTO_INCREMENT,
	source_task_id int NOT NULL,
	target_task_id int NOT NULL,
	lead_time int NOT NULL DEFAULT 0,
	buffer_time int NOT NULL DEFAULT 0,
	coordinates text NOT NULL,
	priority int NOT NULL DEFAULT 1,
	is_forward tinyint(1) NOT NULL DEFAULT '1',
	PRIMARY KEY (id),
	FOREIGN KEY (source_task_id) REFERENCES tasks(id) ON DELETE CASCADE,
	FOREIGN KEY (target_task_id) REFERENCES tasks(id) ON DELETE CASCADE 
);