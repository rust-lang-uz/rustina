import { Context, InlineKeyboard, NextFunction } from "../deps.ts";
import { reply } from "../utils/sender.ts";

export default async (ctx: Context, next: NextFunction) => {
  if (ctx.chat!.id !== -1001518595284) {
    return await reply(
      ctx,
      `⚠️ Bu komanda faqat o'zimizni guruh uchun`,
      new InlineKeyboard().url(
        `Guruhimizga o'ting`,
        `https://t.me/rustlanguz`,
      ),
    );
  }
  await next();
};
