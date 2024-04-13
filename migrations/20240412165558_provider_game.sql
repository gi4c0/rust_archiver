-- Add migration script here
create table if not exists public.provider_game
(
    id        uuid default uuid_generate_v4() not null
        constraint "PK_d2d465ad8bd45aee8716c5ea094"
            primary key,
    vendor_id varchar(50),
    product   varchar(255)                    not null
        constraint "FK_4b1384cdd5f1ee4b92bbb4a9a63"
            references public.product,
    provider  varchar(255)                    not null
        constraint "FK_92fbb0a04cf3d78134ab12135f9"
            references public.provider,
    label     varchar(255)                    not null,
    kind      varchar(50)                     not null
        constraint "FK_02d8997a76f989c8e0488ef2bb1"
            references public.provider_game_kind,
    visible   boolean                         not null,
    constraint "UQ_406b78ef35689f135019a334836"
        unique (provider, label)
);
