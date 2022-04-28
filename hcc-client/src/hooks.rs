use yew::prelude::*;

use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use gloo_events::{EventListener, EventListenerOptions};

use gloo_utils::window;

use gloo_render::{request_animation_frame, AnimationFrame};

use wasm_bindgen::JsValue;

// code inlined from jetli/yew-hooks

// use_is_first_mount.rs
/// A hook returns true if component is just mounted (on first render) and false otherwise.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_is_first_mount;
///
/// #[function_component(IsFirstMount)]
/// fn is_first_mount() -> Html {
///     let is_first = use_is_first_mount();
///     
///     html! {
///         <>
///             { is_first }
///         </>
///     }
/// }
/// ```
pub fn use_is_first_mount() -> bool {
    let is_first = use_mut_ref(|| true);

    if *is_first.borrow_mut() {
        *is_first.borrow_mut() = false;

        return true;
    }

    false
}

// use_mount.rs
/// A lifecycle hook that calls a function after the component is mounted.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_mount;
///
/// #[function_component(Mount)]
/// fn mount() -> Html {
///     use_mount(|| {
///         debug!("Running effect once on mount");
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_mount<Callback>(callback: Callback)
where
    Callback: FnOnce() + 'static,
{
    use_effect_once(move || {
        callback();

        || ()
    });
}

// use_effect_once.rs

/// A lifecycle hook that runs an effect only once.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_effect_once;
///
/// #[function_component(EffectOnce)]
/// fn effect_once() -> Html {
///     use_effect_once(|| {
///         debug!("Running effect once on mount");
///         
///         || debug!("Running clean-up of effect on unmount")
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_effect_once<Callback, Destructor>(callback: Callback)
where
    Callback: FnOnce() -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
{
    use_effect_with_deps(move |_| callback(), ());
}

// use_effect_update.rs
/// This hook ignores the first invocation (e.g. on mount).
/// The signature is exactly the same as the [`use_effect`] hook.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_effect_update;
///
/// #[function_component(UseEffectUpdate)]
/// fn effect_update() -> Html {
///     use_effect_update(|| {
///         debug!("Running effect only on updates");
///
///         || ()
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_effect_update<Callback, Destructor>(callback: Callback)
where
    Callback: FnOnce() -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
{
    let first = use_is_first_mount();

    use_effect(move || {
        if !first {
            Box::new(callback())
        } else {
            Box::new(|| ()) as Box<dyn FnOnce()>
        }
    });
}

/// State handle for the [`use_mut_latest`] hook.
pub struct UseMutLatestHandle<T> {
    inner: Rc<RefCell<Rc<RefCell<T>>>>,
}

impl<T> UseMutLatestHandle<T> {
    /// Get the latest mutable ref to state or props.
    pub fn current(&self) -> Rc<RefCell<T>> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseMutLatestHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseMutLatestHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// State handle for the [`use_latest`] hook.
pub struct UseLatestHandle<T> {
    inner: Rc<RefCell<Rc<T>>>,
}

impl<T> UseLatestHandle<T> {
    /// Get the latest immutable ref to state or props.
    pub fn current(&self) -> Rc<T> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseLatestHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseLatestHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook returns the latest mutable ref to state or props.
///
/// # Example
///
/// ```rust
/// # use gloo::timers::callback::Interval;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_mut_latest;
///
/// #[function_component(UseMutLatest)]
/// fn mut_latest() -> Html {
///     let state = use_state(|| 0);
///     let interval = use_mut_ref(|| None);
///     let closure = {
///         let state = state.clone();
///         move || state.set(*state + 1)
///     };
///
///     let latest_closure = use_mut_latest(closure);
///
///     use_effect_with_deps(move |_| {
///         *interval.borrow_mut() = Some(Interval::new(1000, move || {
///             let latest_closure = latest_closure.current();
///             let closure = &*latest_closure.borrow_mut();
///             // This will get the latest closure and increase state by 1 each time.
///             closure();
///         }));
///         move || *interval.borrow_mut() = None
///     }, ());
///     
///     html! {
///         <div>
///             <p>
///                 <b>{ "Latest value: " }</b>
///                 { *state }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_mut_latest<T>(value: T) -> UseMutLatestHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(RefCell::new(value));
    let inner = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    *inner.borrow_mut() = value_rc;

    UseMutLatestHandle { inner }
}

/// This hook returns the latest immutable ref to state or props.
///
/// # Example
///
/// ```rust
/// # use gloo::timers::callback::Interval;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_latest;
///
/// #[function_component(UseLatest)]
/// fn latest() -> Html {
///     let state = use_state(|| 0);
///     let interval = use_mut_ref(|| None);
///
///     let latest_state = use_latest(state.clone());
///
///     {
///         let state = state.clone();
///         use_effect_with_deps(move |_| {
///             *interval.borrow_mut() = Some(Interval::new(1000, move || {
///                 // This will get the latest state and increase it by 1 each time.
///                 state.set(**latest_state.current() + 1);
///             }));
///             move || *interval.borrow_mut() = None
///         }, ());
///     }
///     
///     html! {
///         <div>
///             <p>
///                 <b>{ "Latest value: " }</b>
///                 { *state }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_latest<T>(value: T) -> UseLatestHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(value);
    let inner = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    *inner.borrow_mut() = value_rc;

    UseLatestHandle { inner }
}

/// A hook that subscribes a callback to events.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_event;
///
/// #[function_component(UseEvent)]
/// fn event() -> Html {
///     let button = use_node_ref();
///
///     use_event(button.clone(), "click", move |_: MouseEvent| {
///         debug!("Clicked!");
///     });
///     
///     html! {
///         <>
///             <button ref={button}>{ "Click me!" }</button>
///         </>
///     }
/// }
/// ```
pub fn use_event<T, F, E>(node: NodeRef, event_type: T, callback: F)
where
    T: Into<Cow<'static, str>>,
    F: Fn(E) + 'static,
    E: From<JsValue>,
{
    let callback = use_latest(callback);

    use_effect_with_deps(
        move |(node, event_type)| {
            let window = window();
            let node = node.get();
            // If we cannot get the wrapped `Node`, then we use `Window` as the default target of the event.
            let target = node.as_deref().map_or(window.deref(), |t| t);

            // We should only set passive event listeners for `touchstart` and `touchmove`.
            // See here: https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener#Improving_scrolling_performance_with_passive_listeners
            let listener = if event_type == "touchstart"
                || event_type == "touchmove"
                || event_type == "scroll"
            {
                Some(EventListener::new(
                    target,
                    event_type.clone(),
                    move |event| {
                        (*callback.current())(JsValue::from(event).into());
                    },
                ))
            } else {
                Some(EventListener::new_with_options(
                    target,
                    event_type.clone(),
                    EventListenerOptions::enable_prevent_default(),
                    move |event| {
                        (*callback.current())(JsValue::from(event).into());
                    },
                ))
            };

            move || drop(listener)
        },
        (node, event_type.into()),
    );
}

/// A hook that subscribes a callback to events only for window.
/// If you want to specify an event target, use [`use_event`].
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_event_with_window;
///
/// #[function_component(UseEvent)]
/// fn event() -> Html {
///     use_event_with_window("keypress", move |e: KeyboardEvent| {
///         debug!("{} is pressed!", e.key());
///     });
///     
///     html! {
///         <>
///             { "Press any key on your awesome keyboard!" }
///         </>
///     }
/// }
/// ```
pub fn use_event_with_window<T, F, E>(event_type: T, callback: F)
where
    T: Into<Cow<'static, str>>,
    F: Fn(E) + 'static,
    E: From<JsValue>,
{
    use_event(NodeRef::default(), event_type, callback);
}

/// A lifecycle hook that calls a function when the component will unmount.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use log::debug;
/// #
/// use yew_hooks::use_unmount;
///
/// #[function_component(Unmount)]
/// fn unmount() -> Html {
///     use_unmount(|| {
///         debug!("Running clean-up of effect on unmount");
///     });
///     
///     html! {
///         <>
///         </>
///     }
/// }
/// ```
pub fn use_unmount<Callback>(callback: Callback)
where
    Callback: FnOnce() + 'static,
{
    let callback_ref = use_mut_latest(Some(callback));

    use_effect_once(move || {
        move || {
            let callback_ref = callback_ref.current();
            let callback = (*callback_ref.borrow_mut()).take();
            if let Some(callback) = callback {
                callback();
            }
        }
    });
}

enum ToggleAction<T> {
    Toggle,
    Reset,
    Set(T),
    SetLeft,
    SetRight,
}

struct UseToggleReducer<T>
where
    T: PartialEq,
{
    value: Rc<T>,
    left: Rc<T>,
    right: Rc<T>,
}

impl<T> Reducible for UseToggleReducer<T>
where
    T: PartialEq,
{
    type Action = ToggleAction<T>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_value = match action {
            ToggleAction::Toggle => {
                if self.value == self.left {
                    self.right.clone()
                } else {
                    self.left.clone()
                }
            }
            ToggleAction::Reset => self.left.clone(),
            ToggleAction::Set(value) => Rc::new(value),
            ToggleAction::SetLeft => self.left.clone(),
            ToggleAction::SetRight => self.right.clone(),
        };

        Self {
            value: next_value,
            left: self.left.clone(),
            right: self.right.clone(),
        }
        .into()
    }
}

impl<T> PartialEq for UseToggleReducer<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// State handle for the [`use_toggle`] hook.
pub struct UseToggleHandle<T>
where
    T: PartialEq,
{
    inner: UseReducerHandle<UseToggleReducer<T>>,
}

impl<T> UseToggleHandle<T>
where
    T: PartialEq,
{
    /// Toggle the value.
    pub fn toggle(&self) {
        self.inner.dispatch(ToggleAction::Toggle)
    }

    /// Set to a value.
    pub fn set(&self, value: T) {
        self.inner.dispatch(ToggleAction::Set(value))
    }

    /// Set to the left default value.
    pub fn set_left(&self) {
        self.inner.dispatch(ToggleAction::SetLeft)
    }

    /// Set to the right other value.
    pub fn set_right(&self) {
        self.inner.dispatch(ToggleAction::SetRight)
    }

    /// Reset to the default value.
    pub fn reset(&self) {
        self.inner.dispatch(ToggleAction::Reset)
    }
}

impl<T> Deref for UseToggleHandle<T>
where
    T: PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl<T> Clone for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl<T: fmt::Debug> fmt::Debug for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseToggleHandle")
            .field("value", &format!("{:?}", self.inner.value))
            .field("left", &format!("{:?}", self.inner.left))
            .field("right", &format!("{:?}", self.inner.right))
            .finish()
    }
}

/// This hook is a simplified [`use_toggle`] to manage boolean toggle state in a function component.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_bool_toggle;
///
/// #[function_component(Toggle)]
/// fn toggle() -> Html {
///     let toggle = use_bool_toggle(true);
///
///     let onclick = {
///         let toggle = toggle.clone();
///         Callback::from(move |_| toggle.toggle())
///     };
///     
///     html! {
///         <div>
///             <button {onclick}>{ "Toggle" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *toggle }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_bool_toggle(default: bool) -> UseToggleHandle<bool> {
    use_toggle(default, !default)
}

/// This hook is used to manage toggle state in a function component.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_toggle;
/// #
/// #[function_component(UseToggle)]
/// fn toggle() -> Html {
///     let toggle = use_toggle("Hello", "World");
///
///     let onclick = {
///         let toggle = toggle.clone();
///         Callback::from(move |_| toggle.toggle())
///     };
///     
///     html! {
///         <div>
///             <button {onclick}>{ "Toggle" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *toggle }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_toggle<T>(default: T, other: T) -> UseToggleHandle<T>
where
    T: 'static + PartialEq,
{
    let value = Rc::new(default);
    let left = value.clone();
    let right = Rc::new(other);

    let inner = use_reducer(move || UseToggleReducer { value, left, right });

    UseToggleHandle { inner }
}

/// A sensor hook that tracks Window scroll position.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_window_scroll;
///
/// #[function_component(UseWindowScroll)]
/// fn window_scroll() -> Html {
///     let state = use_window_scroll();
///     
///     html! {
///         <>
///             <b>{ " X: " }</b>
///             { state.0 }
///             <b>{ " Y: " }</b>
///             { state.1 }
///         </>
///     }
/// }
/// ```

/// State handle for the [`use_raf_state`] hook.
pub struct UseRafStateHandle<T> {
    inner: UseStateHandle<T>,
    raf: Rc<RefCell<Option<AnimationFrame>>>,
}

impl<T> UseRafStateHandle<T>
where
    T: 'static,
{
    /// Replaces the value.
    pub fn set(&self, value: T) {
        let inner = self.inner.clone();
        *self.raf.borrow_mut() = Some(request_animation_frame(move |_| {
            inner.set(value);
        }));
    }
}

impl<T> Deref for UseRafStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl<T> Clone for UseRafStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            raf: self.raf.clone(),
        }
    }
}

impl<T> PartialEq for UseRafStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// A state hook that only updates state in the callback of `requestAnimationFrame`.
///
/// # Example
///
/// ```rust
/// # use web_sys::Window;
/// # use yew::prelude::*;
/// #
/// use yew_hooks::{use_event_with_window, use_raf_state};
///
/// #[function_component(UseRafState)]
/// fn raf_state() -> Html {
///     let state = use_raf_state(|| (0f64, 0f64));
///
///     {
///         let state = state.clone();
///         use_event_with_window("resize", move |e: Event| {
///             let window: Window = e.target_unchecked_into();
///             state.set((
///                 window.inner_width().unwrap().as_f64().unwrap(),
///                 window.inner_height().unwrap().as_f64().unwrap(),
///             ));
///         });
///     }
///     
///     html! {
///         <>
///             <b>{ " Width: " }</b>
///             { state.0 }
///             <b>{ " Height: " }</b>
///             { state.1 }
///         </>
///     }
/// }
/// ```
pub fn use_raf_state<T, F>(init_fn: F) -> UseRafStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let inner = use_state(init_fn);
    let raf = use_mut_ref(|| None);

    {
        let raf = raf.clone();
        use_unmount(move || {
            *raf.borrow_mut() = None;
        });
    }

    UseRafStateHandle { inner, raf }
}

pub fn use_window_scroll() -> (f64, f64) {
    let state = use_raf_state(|| {
        (
            window().page_x_offset().unwrap(),
            window().page_y_offset().unwrap(),
        )
    });

    {
        let state = state.clone();
        use_event_with_window("scroll", move |_: Event| {
            state.set((
                window().page_x_offset().unwrap(),
                window().page_y_offset().unwrap(),
            ));
        });
    }

    {
        let state = state.clone();
        use_mount(move || {
            state.set((
                window().page_x_offset().unwrap(),
                window().page_y_offset().unwrap(),
            ));
        });
    }

    *state
}
