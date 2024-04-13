create table public.error_kind
(
    kind  integer      not null
        constraint "PK_8b63b2f1e1bc9b2feb7730042a3"
            primary key,
    label varchar(100) not null
);
