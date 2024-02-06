use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .col(ColumnDef::new(Events::Queue).string().not_null())
                    .col(ColumnDef::new(Events::Id).string().not_null())
                    .col(ColumnDef::new(Events::TraceId).string().not_null())
                    .col(ColumnDef::new(Events::Event).json().not_null())
                    .col(ColumnDef::new(Events::Meta).json().not_null())
                    .col(ColumnDef::new(Events::Ts).timestamp().not_null())
                    .col(
                        ColumnDef::new(Events::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Events::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(Index::create().col(Events::Queue).col(Events::Id))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("events_trace_id_idx")
                    .table(Events::Table)
                    .col(Events::TraceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Queue,
    Id,
    TraceId,
    Event,
    Meta,
    Ts,
    CreatedAt,
    UpdatedAt,
}
