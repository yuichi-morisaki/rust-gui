use orbtk::prelude::*;
use std::cell::Cell;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::new()
                .title("Counter")
                .position((100.0, 100.0))
                .size(300.0, 300.0)
                .child(MainView::new().build(ctx))
                .build(ctx)
        })
        .run();
}

#[derive(Clone, Copy)]
enum Action {
    Increment,
}

#[derive(Default, AsAny)]
pub struct MainViewState {
    count: Cell<usize>,
    action: Cell<Option<Action>>,
}

widget!(MainView<MainViewState> {
    counter: usize
});

impl MainViewState {
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into())
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::Increment => {
                    let result = self.count.get() + 1;
                    self.count.set(result);
                    ctx.child("text-block")
                        .set("text", String16::from(result.to_string()));
                }
            }

            self.action.set(None);
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Stack::new()
                .child(
                    TextBlock::new()
                        .margin((0.0, 0.0, 0.0, 8.0))
                        .text("0")
                        .style("text-block")
                        .id("text-block")
                        .build(ctx),
                )
                .child(
                    Button::new()
                        .text("count up")
                        .h_align("center")
                        .on_click(move |states, _| -> bool {
                            state(id, states).action(Action::Increment);
                            true
                        })
                        .build(ctx),
                )
                .build(ctx),
        )
    }
}

fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
