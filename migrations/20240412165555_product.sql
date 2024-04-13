-- Add migration script here
create table if not exists public.product
(
    alias    varchar(255) not null
        constraint "PK_afa492329cc228eb3fdd51d86fd"
            primary key,
    label    varchar(100) not null,
    visible  boolean      not null,
    ordering smallint     not null
);
