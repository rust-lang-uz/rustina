import which from "./which.ts";
import { Bot } from "../deps.ts";
import channel from "./channel.ts";
import trigger from "./trigger.ts";
import groups from "./groups.ts";
import useful from "./useful.ts";
import latest from "./latest.ts";
import version from "./version.ts";

export default async (bot: Bot) => {
  await bot
    .use(which)
    .use(groups)
    .use(useful)
    .use(latest)
    .use(version)
    .use(trigger)
    .use(channel);
};
