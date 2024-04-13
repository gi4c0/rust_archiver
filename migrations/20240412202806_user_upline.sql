create table if not exists public.user_upline
(
    user_id    uuid   not null
        constraint "PK_0b46946ab5b6bf20327cf853e76"
            primary key
        constraint "FK_0b46946ab5b6bf20327cf853e76"
            references public."user",
    upline_ids uuid[] not null
);
