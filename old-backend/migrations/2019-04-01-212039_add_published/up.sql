-- Your SQL goes here
alter table posts
    add published boolean default false not null;