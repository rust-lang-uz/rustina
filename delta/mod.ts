import which from "./which.ts";
import { Bot } from "../deps.ts";
import trigger from "./trigger.ts";
import useful from "./useful.ts";

export default async (bot: Bot) => {
  await bot
    .use(which)
    .use(useful)
    .use(trigger)
};
