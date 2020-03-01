use derivative::Derivative;
use serde::{Deserialize, Serialize};
use winit::event::{MouseButton, VirtualKeyCode};

use super::{
    bindings::BindingTypes,
    button::Button,
    controller::{ControllerAxis, ControllerButton},
    scroll_direction::ScrollDirection,
};

/// Events generated by the input system
///
/// Type parameter T is the type assigned to your Actions for your
/// InputBundle or InputHandler.
#[derive(PartialEq, Serialize, Deserialize, Debug, Derivative)]
#[derivative(Clone(bound = ""))]
pub enum InputEvent<T>
where
    T: BindingTypes,
{
    /// A key was pressed down, sent exactly once per key press.
    KeyPressed {
        /// `VirtualKeyCode`, used for semantic info. i.e. "W" was pressed
        key_code: VirtualKeyCode,
        /// Scancode, used for positional info. i.e. The third key on the first row was pressed.
        scancode: u32,
    },
    /// A key was released, sent exactly once per key release.
    KeyReleased {
        /// `VirtualKeyCode`, used for semantic info. i.e. "W" was released
        key_code: VirtualKeyCode,
        /// Scancode, used for positional info. i.e. The third key on the first row was released.
        scancode: u32,
    },
    /// A unicode character was received by the window.  Good for typing.
    KeyTyped(char),
    /// A mouse button was pressed down, sent exactly once per press.
    MouseButtonPressed(MouseButton),
    /// A mouse button was released, sent exactly once per release.
    MouseButtonReleased(MouseButton),
    /// A button was pressed.
    ButtonPressed(Button),
    /// A button was released.
    ButtonReleased(Button),
    /// The mouse pointer moved on screen
    CursorMoved {
        /// The amount the cursor moved horizontally in pixels.
        delta_x: f64,
        /// The amount the cursor moved vertically in pixels.
        delta_y: f64,
    },
    /// The mouse device moved. Use this for any use of the mouse that doesn't involve a standard
    /// mouse pointer.
    MouseMoved {
        /// The amount the mouse moved horizontally.
        delta_x: f32,
        /// The amount the mouse moved vertically.
        delta_y: f32,
    },
    /// The mousewheel was moved in either direction
    MouseWheelMoved(ScrollDirection),
    /// An axis value changed.
    ///
    /// Note that this variant is used for `BindingTypes::Axis`, not a `ControllerAxis`.
    AxisMoved {
        /// The axis that moved on the controller.
        axis: T::Axis,
        /// The amount that the axis moved.
        value: f64,
    },
    /// A controller Axis was moved.
    ///
    /// Note that this variant is used for a `ControllerAxis`, not `BindingTypes::Axis`.
    ControllerAxisMoved {
        /// The id for the controller whose axis moved.
        which: u32,
        /// The axis that moved on the controller.
        axis: ControllerAxis,
        /// The amount that the axis moved.
        value: f32,
    },
    ///  A controller button was pressed.
    ControllerButtonPressed {
        /// The id for the controller whose button was pressed.
        which: u32,
        /// The button that was pressed.
        button: ControllerButton,
    },
    ///  A controller button was released.
    ControllerButtonReleased {
        /// The id for the controller whose button was released.
        which: u32,
        /// The button that was released.
        button: ControllerButton,
    },
    /// New controller was connected.
    ControllerConnected {
        /// The id for the controller connected.
        which: u32,
    },
    /// Controller was disconnected, its id might be reused later.
    ControllerDisconnected {
        /// The id for the controller disconnected.
        which: u32,
    },
    /// The associated action had any related button or combination pressed.
    ///
    /// If a combination is bound to an action, it will be pressed
    /// if all buttons within are pressed.
    ActionPressed(T::Action),
    /// The associated action had any related button or combination released.
    ///
    /// If a combination is bound to an action, it will be released
    /// if any of the buttons within is released while all others are pressed.
    ActionReleased(T::Action),
    /// The associated action has its mouse wheel moved.
    ActionWheelMoved(T::Action),
}
