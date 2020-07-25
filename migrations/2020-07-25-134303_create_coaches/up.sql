CREATE TABLE IF NOT EXISTS  coaches (
    id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    full_name varchar(255) NOT NULL,
	email varchar(255) NOT NULL UNIQUE,
    fuzzy_id varchar(255) NOT NULL UNIQUE,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);