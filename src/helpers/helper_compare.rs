use helpers::{HelperDef};
use registry::{Registry};
use context::{Context, JsonTruthy};
use render::{Renderable, RenderContext, RenderError, Helper};

#[derive(Clone, Copy)]
pub struct CompareHelper;

impl HelperDef for CompareHelper{
    fn call(&self, c: &Context, h: &Helper, r: &Registry, rc: &mut RenderContext) -> Result<(), RenderError> {
        let param1 = h.param(0);
        let param2 = h.param(1);

        if param1.is_none() || param2.is_none() {
            return Err(RenderError::new("Param missing for helper \"compare\""));
        }

        let path = rc.get_path().clone();
        //println!("PATH: {:?}", &path);
        let value1 = c.navigate(&path, &param1.unwrap());
        let value2 = c.navigate(&path, &param2.unwrap());

        let value = value1 == value2;
        //println!("#COMPARE VALUE: {:?} == {:?} | {:?}", value1, value2, value);

        let tmpl = if value { h.template() } else { h.inverse() };
        match tmpl {
            Some(ref t) => t.render(c, r, rc),
            None => Ok(())
        }
    }
}

pub static COMPARE_HELPER: CompareHelper = CompareHelper;