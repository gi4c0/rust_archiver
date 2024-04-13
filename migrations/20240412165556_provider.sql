-- Add migration script here
create table if not exists public.provider
(
    label      varchar(255)                                             not null,
    alias      varchar(255)                                             not null
        constraint "PK_2a98217d647afc14f8592e4a851"
            primary key,
    product    varchar(255)                                             not null
        constraint "FK_b0a8ef871a52c6ba7042ee7e660"
            references public.product,
    visible    boolean             default true not null,
    ordering   smallint            default '1'::smallint                not null,
    currencies character varying[] default '{THB}'::character varying[] not null
)
