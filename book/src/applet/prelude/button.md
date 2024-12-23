# Buttons

In this section, we will walk through 2 Rust examples:

- The `button` example illustrates stateless usage but lets us introduce how to handle events with
  callbacks.
- The `led` example illustrates stateful usage and thus how to access state in callbacks.

## Stateless usage

This example prints to the debug output the new state of a button each time that button changed
state and so for all buttons.

Similarly to LEDs, there is a `button::count()` function to discover the number of buttons available
on the board:

```rust,no_run,noplayground
{{#include button1.rs:count}}
```

We want to listen on events for all available buttons, so we loop over all button indices (starting
at 0):

```rust,no_run,noplayground
{{#include button1.rs:loop}}
```

For each button, we define a handler that prints the new button state to the debug output. The
handler takes the new button state as argument. Since `button::State` implements `Debug`, we simply
use `{state:?}` to print the new state.

```rust,no_run,noplayground
{{#include button1.rs:handler}}
```

We can now start listening for events. This is done by creating a `button::Listener` which will call
the provided handler each time the button changes state. We specify the button we want to listen to
by its index and the handler as a closure.

```rust,no_run,noplayground
{{#include button1.rs:listener}}
```

A listener continues to listen for events until it is dropped. Since we want to indefinitely listen,
we must not drop the listener. A simple way to do that is to leak it. This is equivalent to calling
`core::mem::forget()`.

```rust,no_run,noplayground
{{#include button1.rs:leak}}
```

Finally, we just endlessly wait for callbacks. This step is optional: waiting for callbacks
indefinitely is the implicit behavior when `main` exits. In some way, `main` can be seen as a
callback setup procedure. The `scheduling::wait_for_callback()` function puts the applet to sleep
until a callback is scheduled and `scheduled::wait_indefinitely()` is just an infinite loop around
`wait_for_callback()`.

```rust,no_run,noplayground
{{#include button1.rs:wait}}
```

The final code looks like this:

```rust,no_run
{{#include button1.rs:all}}
```

## Testing

The `host` runner currently has 1 button and is either controlled by typing `button` on a line on
`stdin` (the default) or by clicking the button in the Web UI (with `--interface=web`). With
`--interface=stdin` (the default), one can also type `press` and `release` to control those events
independently. Eventually, the number of buttons will be configurable.

The input and output of a host platform with `--interface=stdio` could look like this (using `<-`
for input and `->` for output):

```text
-> Applet running.
<- button
-> 0.580263: Button 0 has been Pressed.
-> 0.580633: Button 0 has been Released.
<- button
-> 3.308765: Button 0 has been Pressed.
-> 3.309006: Button 0 has been Released.
<- press
-> 4.380741: Button 0 has been Pressed.
<- release
-> 5.436308: Button 0 has been Released.
```

## Stateful usage

This example combines all the LEDs and buttons available on the board by maintaining a dynamic
mapping between them. Initially, all buttons map to the first LED. Each time a button is pressed or
released, the LED it is mapped to is toggled. And when a button is released, it maps to the next LED
(or the first one if it was mapping to the last one).

In particular:

- A single button can toggle all LEDs.
- Multiple buttons can toggle the same LED.
- A button may stay pressed while another button is pressed.
- All buttons eventually toggle all LEDs.

We skip over the setup which doesn't illustrate anything new:

```rust,no_run,noplayground
{{#include button2.rs:setup}}
```

Because buttons dynamically map to LEDs, we need some state to store this information. For each
button, we'll store the index of the LED to which it maps to in its handler. We have to use interior
mutability (in this case `Cell`) because the handler is called as `&self` not `&mut self`[^cell].

```rust,no_run,noplayground
{{#include button2.rs:state}}
```

When defining the button handler, we must move (and thus transfer ownership of) the state we just
created to the handler, such that the handler can read and write the state when called.

```rust,no_run,noplayground
{{#include button2.rs:handler}}
```

When the handler is called, we first toggle the associated LED:

```rust,no_run,noplayground
{{#include button2.rs:toggle}}
```

And then if the button is released, we update the dynamic mapping to point to the next LED (wrapping
if needed):

```rust,no_run,noplayground
{{#include button2.rs:update}}
```

Finally, we create a button listener with the defined handler. And we leak it to continue listening
after it goes out of scope and in particular after `main` returns.

```rust,no_run,noplayground
{{#include button2.rs:listener}}
```

The final code looks like this:

```rust,no_run
{{#include button2.rs:all}}
```

[^cell]: This is because the handler could wait for callbacks itself and thus the handler may be
    reentered. This would essentially copy a mutable reference which is unsound.
