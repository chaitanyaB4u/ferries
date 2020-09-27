CREATE TABLE IF NOT EXISTS  correspondences (
	id varchar(100) NOT NULL,
        from_user_id varchar(100) NOT NULL,
	program_id varchar(100) NOT NULL,
	enrollment_id varchar(100) NOT NULL,
	from_email varchar(100) NOT NULL,
	subject varchar(255) NOT NULL,
	content text,
	in_out varchar(100) NOT NULL,
	status varchar(100) NOT NULL,
	sent_at datetime,
	reply_to varchar(100) NOT NULL,
	error varchar(100) NOT NULL,
	error_reason varchar(100),
	to_send_on datetime NOT NULL,
 	created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  	updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  	PRIMARY KEY (id),
	FOREIGN KEY (from_user_id) REFERENCES users(id),
	FOREIGN KEY (program_id) REFERENCES programs(id),
	FOREIGN KEY (enrollment_id) REFERENCES enrollments(id)
);

CREATE TABLE IF NOT EXISTS  mail_recipients (
	id varchar(100) NOT NULL,
	correspondence_id varchar(100) NOT NULL,
	to_user_id varchar(100),
	to_email varchar(100) NOT NULL,
	to_type varchar(100) NOT NULL,
	PRIMARY KEY (id),
	FOREIGN KEY (correspondence_id) REFERENCES correspondences(id),
	FOREIGN KEY (to_user_id) REFERENCES users(id)

);

