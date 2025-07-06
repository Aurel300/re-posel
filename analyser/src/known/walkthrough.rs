use crate::{dis::show_string, Resources, SCB, SDB, SE};

struct WtContext<'a> {
    res: Resources<'a>,
    output: Vec<WtStep>,
}

pub struct WtStep {
    pub user: bool,
    pub step: String,
    pub comment: Option<String>,
}

impl WtStep {
    fn new(step: String) -> Self {
        Self {
            user: false,
            step,
            comment: None,
        }
    }
}

impl<'a> WtContext<'a> {
    fn new(res: Resources<'a>) -> Self {
        Self {
            res,
            output: Vec::new(),
        }
    }
    /*
    fn assert<'b>(&mut self, f: impl FnOnce(AssertContext<'a, 'b>) -> AssertContext<'a, 'b>) {

    }
    */
    fn assert<'b>(&'b mut self) -> AssertContext<'a, 'b> {
        AssertContext {
            ctx: self,
            first: true,
        }
    }
    fn comment(&mut self, comment: &str) {
        self.output.last_mut().unwrap().comment = Some(comment.to_string());
    }
    fn scene(&mut self, key: &str) {
        self.output.push(WtStep::new(format!("cinematic: scene {}", show_string(key, self.res))));
    }
    fn goto(&mut self, key: &str) {
        self.output.push(WtStep::new(format!("goto: scene {}", show_string(key, self.res))));
        self.output.last_mut().unwrap().user = true;
    }
}

struct AssertContext<'a, 'b> {
    ctx: &'b mut WtContext<'a>,
    first: bool,
}

impl<'a, 'b> AssertContext<'a, 'b> {
    fn add_conjunct(&mut self, conjunct: String) {
        if self.first {
            self.ctx.output.push(WtStep::new(format!("assert {conjunct}")));
            self.first = false;
        } else {
            let add_to = &mut self.ctx.output.last_mut().unwrap().step;
            add_to.push_str("\n  && ");
            add_to.push_str(&conjunct);
        }
    }
    fn eq(mut self, name: &str, value: u32) -> Self {
        if let Some(values) = self.ctx.res.entries.get(name).and_then(|e| e.global.as_ref()).map(|g| &g.values) {
            if let Some(hint) = values.get(&value) {
                self.add_conjunct(format!("{} == {value} {SCB}{hint}{SE}", show_string(name, self.ctx.res)));
                return self;
            } else {
                self.add_conjunct(format!("{} == {value} {SCB}?{SE}", show_string(name, self.ctx.res)));
                return self;
            }
        }
        self.add_conjunct(format!("{} == {value}", show_string(name, self.ctx.res)));
        self
    }
    fn have(mut self, item: &str) -> Self {
        self.add_conjunct(format!("{SDB}inv{SE}.has({})", show_string(item, self.ctx.res)));
        self
    }
}

pub fn chapter1<'a>(res: Resources<'a>) -> Vec<WtStep> {
    let mut c = WtContext::new(res);
    c.scene("10c9");
    c.assert()
        .eq("10c8.1095", 2)
        .have("inv.1214")
        .have("inv.119f")
        .have("inv.121f");
    c.goto("10c8");
    c.assert().eq("10a5.1095", 3);
    c.scene("10a5");
    c.assert().eq("123f.1095", 1);
    c.scene("123f");
    c.scene("106a");
    c.output

    /*
    go to 10c8 when ...
    10c9
      
      combine inv.121d (small knife) with 10c9.10af => 10c9.10af.11d7 = 1
      go to 10c8 when 10c9.10af.11d7 == 1 (exit unlocked)
            and have inv.1214 (black sphere)
            and have inv.119f (William's diary)
            and have inv.121f (jewel box)
    10c8 when 10c8.1095 == 2 (done exploring William's study)
    10a5 when 10a5.1095 == 3 (fainted)
    123f when 123f.1095 == 1 (fainted)
    106a
    */
}
