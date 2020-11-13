alter table discussion_queue add column program_id varchar(100) NOT NULL;
alter table discussion_queue add column program_name varchar(255) NOT NULL;
alter table discussion_queue add column coach_id varchar(100) NOT NULL;
alter table discussion_queue add column coach_name varchar(255) NOT NULL;
alter table discussion_queue add column member_id varchar(100) NOT NULL;
alter table discussion_queue add column member_name varchar(255) NOT NULL;
