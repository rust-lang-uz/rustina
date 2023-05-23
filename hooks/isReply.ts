import { reply } from "../utils/sender.ts";
import { Context, NextFunction } from "../deps.ts";
import topics from "../topics.json" assert { type: "json" };

export default async (ctx: Context, next: NextFunction) => {
  if (!ctx.message?.is_topic_message) {
    return await reply(
      ctx,
      `⚠️ <b>Biz topicda emasmiz bu komandani ishlatish uchun!</b>`,
    );
  }

  if (
    !ctx.message?.reply_to_message ||
    Object.values(topics).includes(ctx.message!.reply_to_message!.message_id)
  ) {
    return await reply(ctx, `↪ Reply bilan ko'rsatingchi habarni!`);
  }
  await next();
};
