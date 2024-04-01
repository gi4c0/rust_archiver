use sqlx::PgPool;

pub async fn create_user_table(pg: &PgPool) {
    sqlx::query(
        r#"
            create table if not exists public."user"
            (
                id               uuid default uuid_generate_v4() not null
                    constraint "PK_cace4a159ff9f2512dd42373760"
                        primary key,
                username         varchar(255)
                    constraint "UQ_78a916df40e02a9deb1c4b75edb"
                        unique,
                position         smallint                        not null,
                parent_id        uuid
                    constraint "FK_acb096eef4d8b5acdd7acbb5c84"
                        references public."user"
                        on delete cascade,
                is_sub           boolean                         not null,
                login            varchar(255)
                    constraint "UQ_a62473490b3e4578fd683235c5e"
                        unique,
                email            varchar(255)
                    constraint "UQ_e12875dfb3b1d92d7d7c5377e22"
                        unique,
                phone            varchar(20),
                registered_at    timestamp with time zone,
                activated_at     timestamp with time zone,
                cas_connected_at timestamp with time zone,
                country          varchar(3),
                salt             varchar(500)                    not null,
                password         varchar(500),
                currency         varchar(10)
            );
        "#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'user' table");

    sqlx::query(
        r#"create index if not exists user_username_idx on public."user" (username varchar_pattern_ops);"#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'user' table");

    sqlx::query(
        r#"create index if not exists user_username_idx
        on public."user" (username varchar_pattern_ops);"#,
    )
    .execute(pg)
    .await
    .expect("Failed to create PG 'user' table");
}
