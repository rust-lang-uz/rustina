import { Composer, Context, InlineKeyboard } from "../deps.ts";

const composer = new Composer();

export const message: string =
  `<b>Assalomu alaykum, hurmatli Rustacean!</b> \n` +
  `\n` +
  `Sizni ko'rib turganimdan bag'oyatda xursandman. ` +
  `Men O'zbek Rust jamiyati tomonidan yaratilgan bot hisoblanib, ` +
  `O'zbek Rust jamiyati foydalanuvchilari uchun foydali resurslarni yetkazish, saqlash va ` +
  `ularni saralash uchun xizmat qilaman.`;

export const keyboard = new InlineKeyboard()
  .url("Jamiyat", "https://t.me/rustlanguz")
  .url("Web Sahifa", "https://rust-lang.uz");

composer.command("start", async (ctx: Context): Promise<void> => {
  await ctx.reply(message, {
    parse_mode: "HTML",
    reply_markup: keyboard,
  });
});

export default composer;
