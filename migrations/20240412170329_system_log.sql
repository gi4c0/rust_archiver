-- Add migration script here
create table public.system_log
(
    id          uuid default uuid_generate_v4() not null
        constraint "PK_fa0b9c6bd88ab76873fcf09f3a5"
            primary key,
    description varchar(2000)                   not null,
    date        timestamp with time zone        not null,
    kind        integer                         not null
        constraint "FK_4c4bb19e1427d86f03176107dd0"
            references public.error_kind,
    payload     text
);
