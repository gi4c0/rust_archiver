-- Add migration script here
create table if not exists public.provider_config
(
    game_provider varchar(255) not null
    constraint "PK_49e30eb371654ab9bfbb63ebb02"
    primary key
    constraint "FK_49e30eb371654ab9bfbb63ebb02"
    references public.provider,
    config        text         not null
)
