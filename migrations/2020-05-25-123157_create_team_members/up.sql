CREATE TABLE users (
	id int NOT NULL AUTO_INCREMENT,
    full_name varchar(255) NOT NULL,
	email varchar(255) NOT NULL UNIQUE,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
    blocked tinyint(1) NOT NULL DEFAULT '0',
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id)
);


CREATE TABLE teams (
	id int NOT NULL AUTO_INCREMENT,
    name varchar(255) NOT NULL,
	fuzzy_id varchar(255) NOT NULL UNIQUE,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id)
);

CREATE TABLE team_members (
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