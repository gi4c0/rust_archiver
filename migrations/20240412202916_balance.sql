create table if not exists public.balance
(
    id               uuid default uuid_generate_v4() not null,
    user_id          uuid                            not null
        constraint "FK_abf63b0d5bfa0266a50e5073954"
            references public."user",
    state            bigint                          not null,
    credit           bigint                          not null,
    credit_available bigint                          not null,
    cash             bigint                          not null,
    cash_available   bigint                          not null,
    currency         varchar(10)                     not null,
    constraint "PK_1f67d2f9dc166ca2b5fbb3539af"
        primary key (id, user_id),
    constraint "UQ_4061a7a3aa761f5e0289f301e9a"
        unique (user_id, currency)
);
