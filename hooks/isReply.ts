import { Context, NextFunction } from "../deps.ts";

export default async (ctx: Context, next: NextFunction) => {
  const replyMessage = ctx.message?.reply_to_message;
  if (!replyMessage) {
    return await ctx.reply(`↪ Reply bilan ko'rsatingchi habarni!`);
  }
  await next();
};
