import { Context, InlineKeyboard, NextFunction } from "../deps.ts";

export default async (ctx: Context, next: NextFunction) => {
  if (ctx.chat!.type !== "private") {
    return await ctx.reply(`⚠️ Bu komanda faqat shaxsiy chat uchun!`, {
      reply_markup: new InlineKeyboard().url(
        `Shaxsiy Chat`,
        `https://t.me/rustlanguz`,
      ),
    });
  }
  await next();
};
