alter table programs add column base_program_id varchar(50);
alter table programs add FOREIGN KEY(base_program_id) references base_programs(id); 