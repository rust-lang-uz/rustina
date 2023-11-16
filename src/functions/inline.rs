use crates_io_api::AsyncClient;
use crates_io_api::CratesQuery;
use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::*;

use crate::utils::inmgr::*;

pub async fn inline(
    bot: Bot,
    crates_client: AsyncClient,
    q: InlineQuery,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if q.query.is_empty() {
        return {
            bot.answer_inline_query(
                q.id,
                vec![InlineQueryResultArticle::new(
                    "101",
                    "Qidirishni boshlang!",
                    InputMessageContent::Text(
                        InputMessageContentText::new(NO_INPUT)
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        };
    }

    let query = CratesQuery::builder()
        .search(q.query.clone())
        .page(1)
        .page_size(50)
        .build();

    let request = crates_client.crates(query).await.unwrap().crates;

    if request.is_empty() {
        return {
            bot.answer_inline_query(
                q.id,
                vec![InlineQueryResultArticle::new(
                    "404",
                    "Xatolik yuz berdi!",
                    InputMessageContent::Text(
                        InputMessageContentText::new(
                            format!("<b>{} ga oid natija mavjud emas!</b>\n{}", 
                            q.query.clone(), "Iltimos, boshqattan ushbu qidirmoqchi bo'lgan paketingiz yozib qidirib ko'ring!")
                        )
                            .parse_mode(ParseMode::Html)
                            .disable_web_page_preview(true),
                    ),
                )
                    .reply_markup(err_keyboard())
                .into()],
            )
            .await?;
            Ok(())
        };
    }

    let mut result: Vec<InlineQueryResult> = Vec::new();

    for c in request.iter() {
        result.push(InlineQueryResult::Article(
            InlineQueryResultArticle::new(
                uuid::Uuid::new_v4(),
                c.name.clone(),
                InputMessageContent::Text(
                    InputMessageContentText::new(view_generate(c))
                        .parse_mode(ParseMode::Html)
                        .disable_web_page_preview(true),
                ),
            )
            .description(c.description.clone().unwrap())
            .url(url::Url::parse(&format!("https://crates.io/crates/{}", c.id)).unwrap())
            .into(),
        ));
    }

    bot.answer_inline_query(q.id, result).send().await?;
    Ok(())
}
