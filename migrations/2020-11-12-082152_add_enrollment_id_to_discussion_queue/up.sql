alter table discussion_queue add column enrollment_id varchar(100) NOT NULL;
alter table discussion_queue add FOREIGN KEY(enrollment_id) references enrollments(id); 