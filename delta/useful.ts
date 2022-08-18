import { Composer, Context, InlineKeyboard } from "../deps.ts";
import {
  categories,
  Category,
  indexer,
  material,
  pager,
} from "../utils/picker.ts";

const composer = new Composer();
const ctxMainMenuText =
  "<b>Rustga oid foydali materiallar:</b>\nAgar o'zingizdan material qo'shmoqchi bo'lsangiz, bizni <a href='https://github.com/rust-lang-uz/rustina/blob/main/source.json'>source.json</a> ni yangilang!";

composer.command("useful", async (ctx: Context): Promise<void> => {
  const keyboard = new InlineKeyboard();

  for (const category of categories()) {
    keyboard.text(
      category.charAt(0).toUpperCase() + category.slice(1).replaceAll("_", " "),
      `category_${category}_1`,
    ).row();
  }

  await ctx.reply(ctxMainMenuText, {
    parse_mode: "HTML",
    reply_markup: keyboard,
    disable_web_page_preview: true,
  });
});

composer.callbackQuery("useful", async (ctx: Context): Promise<void> => {
  const keyboard = new InlineKeyboard();

  for (const category of categories()) {
    keyboard.text(
      category.charAt(0).toUpperCase() + category.slice(1).replaceAll("_", " "),
      `category_${category}_1`,
    ).row();
  }

  await ctx.editMessageText(ctxMainMenuText, {
    parse_mode: "HTML",
    reply_markup: keyboard,
    disable_web_page_preview: true,
  });
});

composer.callbackQuery(/^category_(.*)_(\d+)$/, async (ctx: Context) => {
  const page = Number(ctx.match![2]);
  const category = ctx.match![1] as Category;
  const keyboard = new InlineKeyboard();

  for (const material of pager(category, page)) {
    keyboard.text(
      material.name,
      `material_${page}_${category}_${indexer(category, material)}`,
    ).row();
  }

  if (pager(category, page - 1).length > 0) {
    keyboard.text(`⬅️ Oldingi`, `category_${category}_${page - 1}`);
  }

  if (pager(category, page + 1).length > 0) {
    keyboard.text(`Keyingi ➡️`, `category_${category}_${page + 1}`);
  }

  keyboard.row().text(`🔙 Orqaga`, `useful`);

  await ctx.editMessageText(
    `<b>Siz hozirda ${
      category.charAt(0).toUpperCase() + category.slice(1).replaceAll("_", " ")
    } kategoriyasi ichida turibsiz.</b>\n` +
      `Iltimos, keltirilgan materiallardan birini tanlang...`,
    {
      parse_mode: "HTML",
      reply_markup: keyboard,
      disable_web_page_preview: true,
    },
  );
});

composer.callbackQuery(/^material_(\d+)_(.*)_(\d+)$/, async (ctx: Context) => {
  const keyboard = new InlineKeyboard();
  const page = Number(ctx.match![1]);
  const cat = ctx.match![2] as Category;
  const index = Number(ctx.match![3]);
  const result = material(cat, index);

  if (result) {
    const data = result;

    if (data.link) {
      keyboard.url(`Web Sahifasi`, data.link);
    }

    keyboard.row().text(`🔙 Orqaga`, `category_${cat}_${page}`);

    await ctx.editMessageText(
      `<b>${data.name}</b>` +
        `\n` +
        `\n` +
        `<i>${data.desc}</i>` +
        `\n` +
        `\n` +
        `<b>Ushbu pastdagi tugmacha orqali lavha ga o'tib oling:</b>`,
      {
        parse_mode: "HTML",
        reply_markup: keyboard,
      },
    );
  } else {
    await ctx.editMessageText(`<b>Ushbu material mavjud emas!</b>`, {
      parse_mode: "HTML",
      reply_markup: new InlineKeyboard().text(
        `🔙 Orqaga`,
        `category_${cat}_${page}`,
      ),
    });
  }
});

export default composer;
