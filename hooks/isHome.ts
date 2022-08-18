import { Context, InlineKeyboard, NextFunction } from "../deps.ts";

export default async (ctx: Context, next: NextFunction) => {
  if (ctx.chat!.id !== -1001174263940) {
    return await ctx.reply(`⚠️ Bu komanda faqat o'zimizni guruh uchun`, {
      reply_markup: new InlineKeyboard().url(
        `Guruhimizga o'ting`,
        `https://t.me/rustlanguz`,
      ),
    });
  }
  await next();
};
