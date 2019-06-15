-- This file should undo anything in `up.sql`
alter table posts
    alter column author set default 'admin'::text;
