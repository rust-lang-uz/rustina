import { Composer, Context } from "../deps.ts";
import isGroup from "../hooks/isGroup.ts";
import { reply } from "../utils/sender.ts";

const composer = new Composer();

composer.command("which", isGroup, async (ctx: Context): Promise<void> => {
  const status = (await ctx.getChatMember(ctx!.from!.id)).status;

  const message = `<b>Rustacean ${ctx!.from!.first_name} metrikasi:</b>` +
    `\n` +
    `\n` +
    `<b>Ismi:</b> ${ctx!.from!.first_name} ` + `\n` +
    (ctx?.from?.username && "<b>Username:</b> @" + ctx.from.username + `\n`) +
    (ctx?.chat?.id && `<b>Chat ID:</b> <code>${ctx.chat.id}</code>` + `\n`) +
    `<b>Status:</b> ${status}`;

  await reply(ctx, message);
});

export default composer;
