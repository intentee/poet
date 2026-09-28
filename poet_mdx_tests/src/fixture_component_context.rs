use rhai::CustomType;
use rhai::TypeBuilder;

#[derive(Clone)]
pub struct FixtureComponentContext;

impl CustomType for FixtureComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder.with_name("FixtureComponentContext");
    }
}
