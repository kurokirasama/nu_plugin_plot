use nu_plugin_test_support::PluginTest;
use nu_protocol::{IntoPipelineData, Span, Value};
use nu_plugin_plot::PluginPlot;
use std::sync::Arc;

#[test]
fn test_plot_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut test = PluginTest::new("plot", Arc::new(PluginPlot {}))?;

    let input = Value::list(
        vec![
            Value::test_int(1),
            Value::test_int(2),
            Value::test_int(3),
        ],
        Span::test_data(),
    );

    let output = test.eval_with("plot", input.into_pipeline_data())?;
    let value = output.into_value(Span::test_data())?;

    assert!(value.as_str()?.contains("    ")); // TAB indentation
    Ok(())
}

#[test]
fn test_hist_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut test = PluginTest::new("hist", Arc::new(PluginPlot {}))?;

    let input = Value::list(
        vec![
            Value::test_int(1),
            Value::test_int(1),
            Value::test_int(2),
        ],
        Span::test_data(),
    );

    let output = test.eval_with("hist", input.into_pipeline_data())?;
    let value = output.into_value(Span::test_data())?;

    assert!(value.as_str()?.contains("    ")); // TAB indentation
    Ok(())
}

#[test]
fn test_xyplot_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut test = PluginTest::new("xyplot", Arc::new(PluginPlot {}))?;

    let input = Value::list(
        vec![
            Value::list(vec![Value::test_int(1), Value::test_int(2)], Span::test_data()),
            Value::list(vec![Value::test_int(3), Value::test_int(4)], Span::test_data()),
        ],
        Span::test_data(),
    );

    let output = test.eval_with("xyplot", input.into_pipeline_data())?;
    let value = output.into_value(Span::test_data())?;

    assert!(value.as_str()?.contains("    ")); // TAB indentation
    Ok(())
}
