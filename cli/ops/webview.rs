// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{Deserialize, JsonOp, Value};
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::CoreIsolateState;
use deno_core::ZeroCopyBuf;
use futures::future::FutureExt;
use web_view::{builder, Content, Handle};

pub fn init(i: &mut CoreIsolate, s: &State) {
  i.register_op("op_webview_start", s.stateful_json_op2(op_webview_start));
  i.register_op("op_webview_stop", s.stateful_json_op2(op_webview_stop));
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunArgs {
  title: String,
  url: String,
  width: i32,
  height: i32,
  resizable: bool,
  debug: bool,
}

struct ChildResource {
  child: Handle<()>,
}

fn op_webview_start(
  isolate_state: &mut CoreIsolateState,
  state: &State,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, OpError> {
  println!("{:?}", args);

  let run_args: RunArgs = serde_json::from_value(args)?;

  println!("OK HEREEE");

  state.check_run()?;
  let mut resource_table = isolate_state.resource_table.borrow_mut();

  let _title = run_args.title;
  let url = run_args.url;
  let width = run_args.width;
  let height = run_args.height;
  let resizable = run_args.resizable;
  let debug = run_args.debug;

  let child = builder()
    .content(Content::Url(url))
    .title("title")
    .size(width, height)
    .resizable(resizable)
    .debug(debug)
    .user_data(())
    .invoke_handler(move |_webview, _arg| Ok(()))
    .build()?;

  let child_resource = ChildResource {
    child: child.handle(),
  };

  let child_rid = resource_table.add("child", Box::new(child_resource));

  let fut = async move {
    println!("OK");
    child.run()?;
    Ok(json!({ "rid": child_rid }))
  };

  Ok(JsonOp::Async(fut.boxed_local()))
}

#[derive(Deserialize)]
struct WebviewStopArgs {
  rid: i32,
}

fn op_webview_stop(
  isolate_state: &mut CoreIsolateState,
  state: &State,
  args: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<JsonOp, OpError> {
  let args: WebviewStopArgs = serde_json::from_value(args)?;
  let rid = args.rid as u32;

  state.check_run()?;
  let resource_table = isolate_state.resource_table.clone();

  let future = async move {
    let mut resource_table = resource_table.borrow_mut();

    let child_resource = resource_table
      .get_mut::<ChildResource>(rid)
      .ok_or_else(OpError::bad_resource_id)?;

    let child = &mut child_resource.child;

    child.dispatch(move |webview| {
      webview.exit();
      Ok(())
    })?;

    Ok(json!({
       "status": 1,
    }))
  };

  Ok(JsonOp::Async(future.boxed_local()))
}
