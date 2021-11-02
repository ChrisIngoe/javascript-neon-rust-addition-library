use neon::context::{Context, ModuleContext, FunctionContext};
use neon::types::JsNumber;
use neon::result::JsResult;
use neon::result::NeonResult;

fn add_ten(n: i32) -> i32 {
	return n + 10
}

fn add_hundred(n: i32) -> i32 {
	return n + 100
}

fn add_thousand(n: i32) -> i32 {
	return n + 1000
}

fn add_ten_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
	let handle = cx.argument::<JsNumber>(0).unwrap();
	let res = add_ten(handle.value(&mut cx) as i32);
	Ok(cx.number(res))
}

fn add_hundred_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
	let handle = cx.argument::<JsNumber>(0).unwrap();
	let res = add_hundred(handle.value(&mut cx) as i32);
	Ok(cx.number(res))
}

fn add_thousand_api(mut cx: FunctionContext) -> JsResult<JsNumber> {
	let handle = cx.argument::<JsNumber>(0).unwrap();
	let res = add_thousand(handle.value(&mut cx) as i32);
	Ok(cx.number(res))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
	cx.export_function("add_ten", add_ten_api)?;
	cx.export_function("add_hundred", add_hundred_api)?;
	cx.export_function("add_thousand", add_thousand_api)?;
	Ok(())
}