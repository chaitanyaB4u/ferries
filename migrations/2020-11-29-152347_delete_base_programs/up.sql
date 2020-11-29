-- Your SQL goes here
alter table programs drop foreign key `programs_ibfk_2`;
alter table programs drop column base_program_id;
drop table if exists base_program_coaches;
drop table if exists base_programs;

