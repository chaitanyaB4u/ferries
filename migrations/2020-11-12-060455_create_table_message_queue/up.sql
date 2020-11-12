CREATE TABLE IF NOT EXISTS  discussion_queue (
	id varchar(50) NOT NULL,
	to_id varchar(50) NOT NULL,
    discussion_id varchar(50) NOT NULL,
    created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_pending tinyint(1) NOT NULL DEFAULT '1',
  	PRIMARY KEY (id),        
    FOREIGN KEY (to_id) REFERENCES users(id),
    FOREIGN KEY (discussion_id) REFERENCES discussions(id)
);
