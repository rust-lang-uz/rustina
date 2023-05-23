import { Composer, Context, InlineKeyboard } from "../deps.ts";
import { finder, pager as generator } from "../utils/generator.ts";
import hecker from "../utils/checker.ts";
import { reply } from "../utils/sender.ts";

const composer = new Composer();
const ctxMenuText = "<b>Rust Dasturlash tili versiyalari:</b>";

composer.command("version", async (ctx: Context): Promise<void> => {
  const keyboard = new InlineKeyboard();

  for (const release of await generator(1)) {
    keyboard.text(
      release.tag_name,
      `changelog_${1}_${release.id}`,
    ).row();
  }

  if ((await generator(2)).length > 0) {
    keyboard.text(`Keyingi ➡️`, `version_2`);
  }

  await reply(ctx, ctxMenuText, keyboard, {
    disable_web_page_preview: true,
  });
});

composer.callbackQuery(/^version_(\d+)$/, async (ctx: Context) => {
  const page = Number(ctx.match![1]);
  const keyboard = new InlineKeyboard();

  for (const release of await generator(page)) {
    keyboard.text(
      release.tag_name,
      `changelog_${page}_${release.id}`,
    ).row();
  }

  if (page > 1) {
    keyboard.text(`⬅️ Oldingi`, `version_${page - 1}`);
  }

  if ((await generator(page + 1)).length > 0) {
    keyboard.text(`Keyingi ➡️`, `version_${page + 1}`);
  }

  await ctx.editMessageText(ctxMenuText, {
    parse_mode: "HTML",
    reply_markup: keyboard,
    disable_web_page_preview: true,
  });
});

composer.callbackQuery(/^changelog_(\d+)_(\d+)$/, async (ctx: Context) => {
  const keyboard = new InlineKeyboard();
  const page = Number(ctx.match![1]);
  const data = await finder(Number(ctx.match![2]));

  keyboard.url(
    `📝 GitHub da o'qish`,
    data.html_url,
  );

  keyboard.row().text(`🔙 Orqaga`, `version_${page}`);

  await ctx.editMessageText(
    `<b><a href="${
      (await hecker(data.tag_name, data.body)).url
    }">${data.name}</a></b>` +
      `\n` +
      `\n` +
      `<b>Yaratildi:</b> ${
        new Date(data.created_at).toLocaleDateString("uz")
      }` +
      `\n` +
      `<b>E'lon qilindi:</b> ${
        new Date(data.published_at).toLocaleDateString("uz")
      }` +
      `\n` +
      `<b>O'rnatish:</b> <code>rustup install ${data.tag_name}</code>` +
      `\n` +
      `\n` +
      `<b>"Instant view" yoki quyidagi tugma orqali ko'proq ma'lumot oling:</b>`,
    {
      parse_mode: "HTML",
      reply_markup: keyboard,
    },
  );
});

export default composer;
