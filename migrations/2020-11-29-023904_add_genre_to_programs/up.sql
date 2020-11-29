alter table programs add column genre_id varchar(50);
alter table programs add FOREIGN KEY(genre_id) references program_genres(id); 