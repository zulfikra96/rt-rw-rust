-- This file should undo anything in `up.sql`
alter table users drop duedate;
alter table users drop mikrotik_id;
alter table users drop password_plain;