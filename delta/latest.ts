import { Composer, Context, InlineKeyboard } from "../deps.ts";
import type { Release } from "../types/Github.d.ts";
import { last } from "../utils/generator.ts";
import hecker from "../utils/checker.ts";
import { reply } from "../utils/sender.ts";

const composer = new Composer();

export const message = async (data: Release) =>
  // make a message about the release date of the new release
  `<b>Hozirgi eng oxirgi versiya bu <a href="${
    (await hecker(data.tag_name, data.body)).url
  }">${data.tag_name}</a> va ushbu reliz </b> <code>${
    new Date(data.published_at).toLocaleDateString("uz")
  }</code> da e'lon qilingan <a href="${data.author.html_url}">${data.author.login}</a> tomonidan` +
  `\n` +
  `\n` +
  `<b>Ushbu oxirgi relizni o'rnatish uchun terminalingizda</b> <code>rustup update</code> <b>buyrug'ini ishga tushuring!</b>`;

export const keyboard = (data: Release) =>
  new InlineKeyboard().url("Ko'proq ma'lumotlar", data.html_url);

composer.command("last", async (ctx: Context): Promise<void> => {
  const req = await last();
  await reply(ctx, await message(req), keyboard(req));
});

export default composer;
