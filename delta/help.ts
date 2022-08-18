import { Composer, Context } from "../deps.ts";
import * as start from "./start.ts";

const composer = new Composer();

export const message = `<b>Mavjud komandalar ro'yxati:</b>` +
  `\n` +
  `\n` +
  `/doc - <code>reply qilingan odamga dok borligi haqida eslatish</code>` +
  `\n` +
  `/off - <code>reply qilingan odamga offtop borligi haqida eslatish</code>` +
  `\n` +
  `/group - <code>rust ga oid guruh va hamjamiyatlar</code>` +
  `\n` +
  `/help - <code>ushbu xabarni qayta ko'rsatish</code>` +
  `\n` +
  `/about - <code>ushbu botimizning rivojlantirish qismi</code>` +
  `\n` +
  `/rules - <code>qoidalarni aks ettirish</code>` +
  `\n` +
  `/which - <code>ushbu guruh va foydalanuvchi metrik ma'lumotlarini ko'rsatish</code>`;
export const keyboard = start.keyboard;

composer.command("help", async (ctx: Context): Promise<void> => {
  await ctx.reply(message, {
    parse_mode: "HTML",
    reply_markup: keyboard,
  });
});

export default composer;
