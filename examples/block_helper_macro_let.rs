use handlebars::{
    BlockParamHolder, Context, Handlebars, Helper, Output, RenderContext, RenderError,
    RenderErrorReason,
};
use serde_json::{json, Value};

// a custom block helper to bind a variable name to a value
pub fn helper_let<'a>(
    h: &Helper<'a>,
    _r: &'a Handlebars,
    _ctx: &'a Context,
    rc: &'a mut RenderContext,
    _out: &mut dyn Output,
) -> Result<(), RenderError> {
    let name_param = h
        .param(0)
        .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("let", 0))?;

    let Some(Value::String(name_constant)) = name_param.try_get_constant_value() else {
        return Err(RenderErrorReason::ParamTypeMismatchForName(
            "let",
            "0".to_string(),
            "constant string".to_string(),
        )
        .into());
    };

    let value = h
        .param(1)
        .as_ref()
        .map(|v| v.value().to_owned())
        .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("let", 2))?;

    let block = rc.block_mut().unwrap();

    block.set_block_param(
        name_constant.as_str().into(),
        BlockParamHolder::Value(value),
    );

    Ok(())
}

fn main() -> Result<(), RenderError> {
    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("let", Box::new(helper_let));

    let input = r#"
{{#if foo}}
{{let "mixin_classes" "foo-bar baz"}}
{{else}}
{{let "mixin_classes" "quux"}}
{{/if}}

<div class="content-main {{mixin_classes}}">
    <p> content </p>
</div>
"#;

    println!(
        "{}",
        handlebars.render_template(input, &json!({"foo": true}))?
    );

    Ok(())
}
