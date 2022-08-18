import start from "./start.ts";
import help from "./help.ts";
import inline from "./inline.ts";
import which from "./which.ts";
import { Bot } from "../deps.ts";
import about from "./about.ts";
import rules from "./rules.ts";
import channel from "./channel.ts";
import trigger from "./trigger.ts";

export default async (bot: Bot) => {
  await bot
    .use(start)
    .use(help)
    .use(inline)
    .use(which)
    .use(about)
    .use(rules)
    .use(trigger)
    .use(channel);
};
