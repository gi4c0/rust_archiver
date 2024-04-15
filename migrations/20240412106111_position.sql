-- Add migration script here
create table if not exists public.position
(
    level    smallint     not null
        constraint "PK_a82887f0343233638b3c907e012"
            primary key,
    position varchar(255) not null
        constraint "UQ_108b7a06fc0c34b3baff9f3f257"
            unique
);
