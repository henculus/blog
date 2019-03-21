-- Your SQL goes here

insert into users
  (username, password_hash)
select 'admin', 'unusable_password'
where not exists(
    select username from users where username = 'admin'
  );

alter table posts
  add author text default 'admin' not null;

alter table posts
  add constraint posts_users_username_fk
    foreign key (author) references users
      on update cascade on delete cascade;