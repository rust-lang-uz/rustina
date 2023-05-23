import { Context, NextFunction } from "../deps.ts";
import { reply } from "../utils/sender.ts";

export default async (ctx: Context, next: NextFunction) => {
  const replyMessage = ctx.message?.reply_to_message;
  if (!replyMessage) {
    return await reply(ctx, `↪ Reply bilan ko'rsatingchi habarni!`);
  }
  await next();
};
