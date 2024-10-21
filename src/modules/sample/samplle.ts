class Ctx {
    req: string;
    constructor(req: string){
        this.req = req;
    }
}

class ModelOne {
    ctx: Ctx;
    constructor(ctx: Ctx){
        this.ctx = ctx;
    }
}

class ModelSecond {
    ctx: Ctx;
    constructor(ctx: Ctx){
        this.ctx = ctx;
    }
}

class View {
    ctx: Ctx;
    constructor(ctx: Ctx){
        this.ctx = ctx;
    }
}

class Ctrl {
    ctx: Ctx;
    modelOne: ModelOne;
    modelSecond: ModelSecond;

    constructor(req: string){
        this.ctx = new Ctx(req);
        this.modelOne = new ModelOne(this.ctx);
        this.modelSecond = new ModelSecond(this.ctx);
    }
}