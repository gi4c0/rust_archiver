-- Add migration script here
create table if not exists public.provider_game_config
(
    game_id uuid not null
    constraint "PK_4ba0c714256899c77f4ab2176f8"
    primary key
    constraint "FK_4ba0c714256899c77f4ab2176f8"
    references public.provider_game,
    config  text not null
);
