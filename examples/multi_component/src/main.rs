use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone)]
struct State {
    count: u32,
}

struct App {
    /// Our local version of state.
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
}

enum Msg {
    /// Message to receive new state.
    State(Rc<State>),
    Increment,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
            Msg::Increment => {
                self.dispatch.reduce(|s| s.count += 1);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // We use self.link.callback here because onclick = dispatch_withcallback appears
        // to not be working at this time.
        html! {
            <>
                <CountApp/>
                <CountApp/>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{"+1"}</button>
            </>
        }
    }
}

struct CountApp {
    state: Rc<State>,
    _dispatch: Dispatch<BasicStore<State>>,
}

enum CountMsg {
    /// Message to receive new state.
    State(Rc<State>),
}

impl Component for CountApp {
    type Message = CountMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(CountMsg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CountMsg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let count = self.state.count;
        html! {
            <h1>{ count }</h1>
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}
