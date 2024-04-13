create table if not exists public.provider_game_kind
(
    alias    varchar(50)  not null
        constraint "PK_19ceef9d735203ddd9380aad3f2"
            primary key,
    label    varchar(100) not null,
    ordering integer      not null
);
