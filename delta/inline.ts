// deno-lint-ignore-file no-explicit-any
import { api, Composer, Context, InlineKeyboard } from "../deps.ts";
import encoder from "../utils/encoder.ts";

const composer = new Composer();

composer.inlineQuery(/(.*)/ig, async (ctx: Context): Promise<any> => {
  if (ctx.inlineQuery?.query) {
    const request = await api.search(ctx.inlineQuery?.query, 50);

    if (request.meta.total === 0) {
      return await ctx.answerInlineQuery([{
        type: "article",
        id: "404",
        title: "Xatolik yuz berdi!",
        description: `Ushbu ${ctx.inlineQuery?.query} ga oid natija topilmadi!`,
        reply_markup: new InlineKeyboard().switchInlineCurrent(
          "Qayta urinib ko'ramizmi?",
          "rand",
        ),
        input_message_content: {
          message_text:
            `<b>"${ctx.inlineQuery?.query}" ga oid natija mavjud emas!</b>` +
            `\n` +
            `Iltimos, boshqattan ushbu qidirmoqchi bo'lgan paketingiz yozib qidirib ko'ring!`,
          parse_mode: "HTML",
          disable_web_page_preview: true,
        },
      }]);
    }

    return await ctx.answerInlineQuery(
      request.crates.map((item) => ({
        type: "article",
        id: crypto.randomUUID(),
        title: item.name,
        url: `https://crates.io/crates/${item.id}`,
        description: item.description,
        reply_markup: (() => {
          const keyboard = new InlineKeyboard();
          keyboard.url(`Crate`, `https://crates.io/crates/${item.name}`);
          if (item.homepage) keyboard.url(`Asosiy`, item.homepage).row();
          if (item.documentation) {
            keyboard.url(`Dokumentatsiya`, item.documentation).row();
          }
          if (item.repository) keyboard.url(`Repozitoriya`, item.repository);
          return keyboard;
        })(),
        input_message_content: {
          message_text: `<b>🦀 Rust Telegram Cratelari 🦀</b>\n\n` +
            `📦 <b>Nomi:</b> ${item.name}` +
            `\n` +
            `🚨 <b>Oxirgi versiya:</b> <code>${item.newest_version}</code> \n` +
            `🎚 <b>Yuklab olingan:</b> yaqin orada: <code>${item.recent_downloads}</code> | hammasi: <code>${item.downloads}</code> \n` +
            `⌚️ <b>Yaratilgan:</b> <code>${
              new Date(item.created_at).toLocaleString("uz")
            }</code> \n` +
            `📡 <b>Yangilangan:</b> <code>${
              new Date(item.updated_at).toLocaleString("uz")
            }</code> \n` +
            `📰 <b>Ma'lumot:</b> <code>${
              (encoder(item.description)).substring(0, 100)
            }${item.description.length > 100 ? "..." : ""}</code> \n\n` +
            `🔌 <b>Cargo.toml fayliga qo'shib qo'ying:</b> \n` +
            `<code>[dependencies]</code>\n<code>${item.name} = "${item.max_stable_version}"</code>`,
          parse_mode: "HTML",
          disable_web_page_preview: true,
        },
      })),
      { cache_time: 1 }, // { cache_time: 24 * 3600 },
    );
  }

  if (!ctx.inlineQuery?.query) {
    return await ctx.answerInlineQuery([{
      type: "article",
      id: "101",
      title: "Qidirishni boshlang!",
      description: "Qidirmoqchi bo'lgan paketingiz nomini yozing!",
      reply_markup: new InlineKeyboard().switchInlineCurrent(
        "Qayta urinib ko'ramizmi?",
        "rand",
      ),
      input_message_content: {
        message_text: `<b>Salom foydalanuvchi!</b>` +
          `\n` +
          `Siz inline rejim ishga tushurdingiz. Ushbu qulaylik yordamida siz crates.io ` +
          `Rust dasturlash tili paketlar registridan web sahifani ishlatmasdan turib ` +
          `telegram o'zida kerakli paketlarni qidirishingiz mumkin! Ushbu qulaylik ` +
          `yozish uchun o'zimizning <a href="https://github.com/rust-lang-uz/crates.ts">API SDK</a> ishlatildi` +
          `Qidirishni boshlash uchun: ` +
          `\n` +
          `<code>@rustaceanbot &lt;nomi&gt;</code>`,
        parse_mode: "HTML",
        disable_web_page_preview: true,
      },
    }]);
  }
});

export default composer;
