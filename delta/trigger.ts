import { Composer, Context, InlineKeyboard } from "../deps.ts";
import isReply from "../hooks/isReply.ts";

const composer = new Composer();

composer.command("off", isReply, async (ctx: Context): Promise<void> => {
  if (ctx?.message?.reply_to_message?.from?.id === ctx.me.id) {
    await ctx.reply(`Ha-ha... yaxshi urinish!`, {
      parse_mode: "HTML",
    });
  } else {
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.reply_to_message!.message_id,
    );
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.message_id,
    );
    await ctx.reply(
      `<b>Hurmatli <a href="tg://user?id=${ctx?.message?.reply_to_message?.from?.id}">${ctx?.message?.reply_to_message?.from?.first_name}</a>,</b>` +
        `\n` +
        `\n` +
        `Tushunishim bo'yicha siz mavzudan chetlayashayabsiz. Iltimos, ` +
        `quyidagi tugmachani bosish orqali bizning offtop guruhga o'tib oling! ` +
        `Offtopic guruhimizda istalgan mavzuda suhbatlashish ruxsat etiladi. Boshqalarga halaqit qilmayliga 😉` +
        `\n` +
        `\n` +
        `<b>Hurmat ila, Rustina (Rastina)</b>`,
      {
        parse_mode: "HTML",
        reply_markup: new InlineKeyboard().url(
          `Offtop Chat`,
          `https://t.me/rustlanguz_offtopic`,
        ),
      },
    );
  }
});

composer.command("nometa", isReply, async (ctx: Context): Promise<void> => {
  if (ctx?.message?.reply_to_message?.from?.id === ctx.me.id) {
    await ctx.reply(`Ha-ha... yaxshi urinish!`, {
      parse_mode: "HTML",
    });
  } else {
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.reply_to_message!.message_id,
    );
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.message_id,
    );
    await ctx.reply(
      `<b>Hurmatli <a href="tg://user?id=${ctx?.message?.reply_to_message?.from?.id}">${ctx?.message?.reply_to_message?.from?.first_name}</a>,</b>` +
      `\n` +
      `\n` +
      `Tushunishim bo'yicha siz boshqalarni vaqtini isrof qilayabsiz. Iltimos, ` +
      `quyidagi tugmachani bosish orqali to'g'ri savol berishni o'rganing! ` +
      `Boshqalarni ham sizni kutgani ko'p vaqti yo'q. Hammaning vaqtini hurmat qilayliga 😉` +
      `\n` +
      `\n` +
      `<b>Hurmat ila, Rustina (Rastina)</b>`,
      {
        parse_mode: "HTML",
        reply_markup: new InlineKeyboard().url(
          `Nometa Qo'llanmasi`,
          `https://nometa.uz/`,
        ),
      },
    );
  }
});

composer.command("nonoff", isReply, async (ctx: Context): Promise<void> => {
  if (ctx?.message?.reply_to_message?.from?.id === ctx.me.id) {
    await ctx.reply(`Ha-ha... yaxshi urinish!`, {
      parse_mode: "HTML",
    });
  } else {
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.reply_to_message!.message_id,
    );
    await ctx.api.deleteMessage(
      ctx.message!.chat!.id,
      ctx.message!.message_id,
    );
    await ctx.reply(
      `<b>Hurmatli <a href="tg://user?id=${ctx?.message?.reply_to_message?.from?.id}">${ctx?.message?.reply_to_message?.from?.first_name}</a>,</b>` +
        `\n` +
        `\n` +
        `Chunishim bo'yicha siz mavzuga kirib ketayabsiz. Iltimos, ` +
        `quyidagi tugmachani bosish orqali bizning asosiy guruhga o'tib oling! ` +
        `Linux haqida iloji boricha asosiy guruhimizda suhbatlashish tavsiya etiladi. Offtopchilarga halaqit qilmayliga 😉` +
        `\n` +
        `\n` +
        `<b>Hurmat ila, Rustina (Rastina)</b>`,
      {
        parse_mode: "HTML",
        reply_markup: new InlineKeyboard().url(
          `Asosiy Chat`,
          `https://t.me/rustlanguz`,
        ),
      },
    );
  }
});

composer.command("doc", isReply, async (ctx: Context): Promise<void> => {
  if (ctx?.message?.reply_to_message?.from?.id === ctx.me.id) {
    await ctx.reply(`Ha-ha... yaxshi urinish!`, {
      parse_mode: "HTML",
    });
  } else {
    await ctx.reply(
      `<b>Demak, <a href="tg://user?id=${ctx?.message?.reply_to_message?.from?.id}">${ctx?.message?.reply_to_message?.from?.first_name}</a>,</b>` +
        `\n` +
        `\n` +
        `<i>Bir bor ekan, bir yo'q ekan... Qadim o'tgan zamonlarda dokumentatsiya ` +
        `bo'lgan ekan. Aytishlariga qaraganda, undan deyarli hamma savollarga ` +
        `javob olsa bo'larkanda. Yanachi, unga avtorlar shunchalik ko'p vaqt ajratishar ` +
        `ekanu, lekin uni sanoqligina odam o'qisharkan. Attang...</i>`,
      {
        parse_mode: "HTML",
      },
    );
  }
});

export default composer;
