-- Todo status enum
create type todo_status_enum as enum(
    'open',
    'close'
);

-- Todo
create table todo(
    id bigserial,
    cid bigint not null,
    ctime timestamp with time zone default now(),
    title text not null,
    status todo_status_enum not null default 'open'
);
alter sequence todo_id_seq restart with 1000;
