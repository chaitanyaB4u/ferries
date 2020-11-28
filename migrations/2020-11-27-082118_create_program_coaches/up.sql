CREATE TABLE IF NOT EXISTS program_genres (
    id varchar(50) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS base_programs (
	id varchar(50) NOT NULL,
    name varchar(255) NOT NULL,
    description text,
    genre_id varchar(50) NOT NULL,
    active tinyint(1) NOT NULL DEFAULT '1',
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (genre_id) REFERENCES program_genres(id)  
);

CREATE TABLE IF NOT EXISTS base_program_coaches (
	id varchar(50) NOT NULL,
	base_program_id varchar(50) NOT NULL,
    coach_id varchar(100) NOT NULL,
    is_admin tinyint(1) NOT NULL DEFAULT '1',
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
    FOREIGN KEY (base_program_id) REFERENCES base_programs(id),
    FOREIGN KEY (coach_id) REFERENCES coaches(id)
);

