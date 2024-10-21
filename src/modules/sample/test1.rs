// Структура для Ctx
struct Ctx {
    req: String,
}

impl Ctx {
    // "Конструктор" new для создания объекта Ctx
    fn new(req: String) -> Self {
        Ctx { req }
    }
}

// Структура для ModelOne
struct ModelOne<'a> {
    ctx: &'a Ctx,
}

impl<'a> ModelOne<'a> {
    fn new(ctx: &'a Ctx) -> Self {
        ModelOne { ctx }
    }
}

// Структура для ModelSecond
struct ModelSecond<'a> {
    ctx: &'a Ctx,
}

impl<'a> ModelSecond<'a> {
    fn new(ctx: &'a Ctx) -> Self {
        ModelSecond { ctx }
    }
}

// Структура для View (не используется в оригинальном коде, но для полноты переведём)
struct View<'a> {
    ctx: &'a Ctx,
}

impl<'a> View<'a> {
    fn new(ctx: &'a Ctx) -> Self {
        View { ctx }
    }
}

// Структура для Ctrl
struct Ctrl {
    ctx: Ctx,
    model_one: ModelOne<'static>,
    model_second: ModelSecond<'static>,
}

impl Ctrl {
    // "Конструктор" new для создания объекта Ctrl
    fn new(req: String) -> Self {
        // Создаем контекст
        let ctx = Ctx::new(req);

        // Создаем модели, привязываем к контексту
        let model_one = ModelOne::new(&ctx);
        let model_second = ModelSecond::new(&ctx);

        // Возвращаем созданный объект Ctrl
        Ctrl {
            ctx,
            model_one,
            model_second,
        }
    }
}

fn main() {
    // Пример использования
    let ctrl = Ctrl::new(String::from("example request"));
    println!("Ctrl created with request: {}", ctrl.ctx.req);
}
