create table if not exists public.currency
(
    label      varchar(10) not null
        constraint "PK_8e05f18b5e44565959b408e6d39"
            primary key,
    rate       integer     not null,
    active     boolean     not null,
    ordering   smallint    not null,
    is_default boolean     not null
);
