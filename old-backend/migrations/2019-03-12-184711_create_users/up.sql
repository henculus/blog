-- Your SQL goes here
create table users
(
  username      text primary key,
  password_hash text   not null,
  user_roles    text[] not null default '{"user"}'
);