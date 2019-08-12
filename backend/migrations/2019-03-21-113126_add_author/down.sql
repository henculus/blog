-- This file should undo anything in `up.sql`
alter table posts
  drop constraint posts_users_username_fk;

alter table posts
  drop column author;
