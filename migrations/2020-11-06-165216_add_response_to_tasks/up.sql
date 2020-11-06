-- Your SQL goes here
alter table tasks add column response text;
alter table tasks add column approved_at datetime;
alter table tasks add column cancelled_at datetime;

