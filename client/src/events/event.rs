use sdl2::event::Event;

#[derive(Debug)]
pub enum UserEvent {
    /// Indicates a mouse entered the bounds of an object.  The ID of the object is returned.
    EnteredBounds(u32),
    /// Indicates a mouse exited the bounds of an object.  The ID of the object is returned.
    ExitedBounds(u32),
    /// Indicates an object was clicked.  The ID of the object is returned along with the number
    /// of clicks within the object.
    Clicked(u32, u32), 
    /// Indicates an SDL-based Event occurred.
    SystemEvent(Event),
}

/// The `EventHandler` is a class used to handle events generate from the `Engine::run` loop.
/// These are `PushrodEvent` objects, which are events generated from `Widget`s that intercept
/// normal events from `SDL2`.
pub trait EventHandler {
    /// Handles processing of events.  The `events` passed in are references to `PushrodEvent` objects.
    /// This function will only be called if the event list is non-empty.
    fn process_event(&self, events: Vec<&PushrodEvent>);
}