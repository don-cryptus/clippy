use common::{
    constants::{
        DISPLAY_SCALE, DISPLAY_SCALE_MAX, DISPLAY_SCALE_MIN, MAX_FILE_SIZE, MAX_FILE_SIZE_MAX,
        MAX_FILE_SIZE_MIN, MAX_HTML_SIZE, MAX_HTML_SIZE_MAX, MAX_HTML_SIZE_MIN, MAX_IMAGE_SIZE,
        MAX_IMAGE_SIZE_MAX, MAX_IMAGE_SIZE_MIN, MAX_RTF_SIZE, MAX_RTF_SIZE_MAX, MAX_RTF_SIZE_MIN,
        MAX_TEXT_SIZE, MAX_TEXT_SIZE_MIN,
    },
    language::get_system_language,
    types::enums::{ClippyPosition, Language},
};
use sea_orm::Iterable;
use sea_orm_migration::{
    prelude::*,
    schema::{boolean, float, integer, pk_auto, string},
};

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    Language,
    //
    Startup,
    Synchronize,
    Tooltip,
    DarkMode,
    DisplayScale,
    Position,
    //
    MaxFileSize,
    MaxImageSize,
    MaxTextSize,
    MaxRtfSize,
    MaxHtmlSize,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(pk_auto(Settings::Id))
                    .col(
                        string(Settings::Language)
                            .string_len(2)
                            .default(get_system_language().to_string())
                            .check(
                                Expr::col(Settings::Language).is_in(
                                    Language::iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>(),
                                ),
                            ),
                    )
                    .col(boolean(Settings::Startup).default(true))
                    .col(boolean(Settings::Synchronize).default(false))
                    .col(boolean(Settings::DarkMode).default(true))
                    .col(boolean(Settings::Tooltip).default(true))
                    .col(
                        float(Settings::DisplayScale)
                            .default(Value::Float(Some(DISPLAY_SCALE)))
                            .check(
                                Expr::col(Settings::DisplayScale)
                                    .gte(Value::Float(Some(DISPLAY_SCALE_MIN)))
                                    .and(
                                        Expr::col(Settings::DisplayScale)
                                            .lte(Value::Float(Some(DISPLAY_SCALE_MAX))),
                                    ),
                            ),
                    )
                    .col(
                        string(Settings::Position)
                            .default(ClippyPosition::Cursor.to_string())
                            .check(
                                Expr::col(Settings::Position).is_in(
                                    ClippyPosition::iter()
                                        .map(|x| x.to_string())
                                        .collect::<Vec<String>>(),
                                ),
                            ),
                    )
                    // 10MB default, 0 min, 100MB max
                    .col(
                        integer(Settings::MaxFileSize).default(MAX_FILE_SIZE).check(
                            Expr::col(Settings::MaxFileSize)
                                .gte(MAX_FILE_SIZE_MIN)
                                .and(Expr::col(Settings::MaxFileSize).lte(MAX_FILE_SIZE_MAX)),
                        ),
                    )
                    .col(
                        integer(Settings::MaxImageSize)
                            .default(MAX_IMAGE_SIZE)
                            .check(
                                Expr::col(Settings::MaxImageSize)
                                    .gte(MAX_IMAGE_SIZE_MIN)
                                    .and(Expr::col(Settings::MaxImageSize).lte(MAX_IMAGE_SIZE_MAX)),
                            ),
                    )
                    .col(
                        integer(Settings::MaxTextSize).default(MAX_TEXT_SIZE).check(
                            Expr::col(Settings::MaxTextSize)
                                .gte(MAX_TEXT_SIZE_MIN)
                                .and(Expr::col(Settings::MaxTextSize).lte(MAX_FILE_SIZE_MAX)),
                        ),
                    )
                    .col(
                        integer(Settings::MaxRtfSize).default(MAX_RTF_SIZE).check(
                            Expr::col(Settings::MaxRtfSize)
                                .gte(MAX_RTF_SIZE_MIN)
                                .and(Expr::col(Settings::MaxRtfSize).lte(MAX_RTF_SIZE_MAX)),
                        ),
                    )
                    .col(
                        integer(Settings::MaxHtmlSize).default(MAX_HTML_SIZE).check(
                            Expr::col(Settings::MaxHtmlSize)
                                .gte(MAX_HTML_SIZE_MIN)
                                .and(Expr::col(Settings::MaxHtmlSize).lte(MAX_HTML_SIZE_MAX)),
                        ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}
