//! # Yewdux
//!
//! Shared state containers for [Yew](https://yew.rs/docs/en/) applications.
//!
//! State management in Yew can get cumbersome, especially when you need to give many (potentially
//! isolated) components mutable access to shared state. Normally you would need to write individual
//! properties and callbacks for each component to propagate changes -- too much typing if you as me!
//! Yewdux provides an ergonomic interface for shared state containers. They can be accessed from any
//! component or agent, live for entire application lifetime, and are clone-on-write by default.
//!
//! ## Example
//! ```
//! use std::rc::Rc;
//!
//! use yew::prelude::*;
//! use yewdux::prelude::*;
//!
//! #[derive(Default, Clone)]
//! struct State {
//!     count: u32,
//! }
//!
//! struct App {
//!     /// Our local version of state.
//!     state: Rc<State>,
//!     dispatch: Dispatch<BasicStore<State>>,
//! }
//!
//! enum Msg {
//!     /// Message to receive new state.
//!     State(Rc<State>),
//! }
//!
//! impl Component for App {
//!     type Message = Msg;
//!     type Properties = ();
//!
//!     fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
//!         // Create Dispatch with a bridge that receives new state.
//!         let dispatch = Dispatch::bridge_state(link.callback(Msg::State));
//!         // Magically increment our counter for this example.
//!         // NOTE: Changes aren't immediate! We won't see new state until we receive it in our update
//!         // method.
//!         dispatch.reduce(|s| s.count += 1);
//!
//!         Self {
//!             dispatch,
//!             state: Default::default(),
//!         }
//!     }
//!
//!     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//!         match msg {
//!             Msg::State(state) => {
//!                 // Receive new state and re-render.
//!                 self.state = state;
//!                 true
//!             }
//!         }
//!     }
//!
//!     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//!         false
//!     }
//!
//!     fn view(&self) -> Html {
//!         let count = self.state.count;
//!         // We can modify state with callbacks too!
//!         let incr = self.dispatch.reduce_callback(|s| s.count += 1);
//!
//!         html! {
//!             <>
//!             <h1>{ count }</h1>
//!             <button onclick=incr>{"+1"}</button>
//!             </>
//!         }
//!     }
//! }
//!
//!
//! pub fn main() {
//!     yew::start_app::<App>();
//! }
//! ```

pub mod component;
pub mod dispatch;
mod service;
pub mod store;

pub mod prelude {
    //! Everything you need to use Yewdux.

    pub use yew::agent::HandlerId;
    pub use yew_services::storage::Area;

    pub use crate::component::{StateView, WithDispatch};
    pub use crate::dispatch::{Dispatch, DispatchProps, DispatchPropsMut, Dispatcher};
    pub use crate::store::{
        basic::BasicStore,
        persistent::{Persistent, PersistentStore},
        reducer::{Reducer, ReducerStore},
        Changed, Store, StoreLink,
    };
}
