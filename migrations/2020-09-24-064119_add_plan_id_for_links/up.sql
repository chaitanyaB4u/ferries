alter table master_task_links add column master_plan_id varchar(100) NOT NULL;
alter table master_task_links add FOREIGN KEY(master_plan_id) references master_plans(id);
alter table master_task_links drop column buffer_time;
alter table task_links add column enrollment_id varchar(100) NOT NULL;
alter table task_links add FOREIGN KEY(enrollment_id) references enrollments(id);
alter table task_links drop column buffer_time;