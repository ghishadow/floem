use glazier::{kurbo::Point, KeyEvent, MouseEvent};

#[derive(Hash, PartialEq, Eq)]
pub enum EventListner {
    KeyDown,
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    MouseWheel(MouseEvent),
    KeyDown(KeyEvent),
}

impl Event {
    pub fn needs_focus(&self) -> bool {
        match self {
            Event::MouseDown(_)
            | Event::MouseUp(_)
            | Event::MouseMove(_)
            | Event::MouseWheel(_) => false,
            Event::KeyDown(_) => true,
        }
    }

    pub fn point(&self) -> Option<Point> {
        match self {
            Event::MouseDown(mouse_event)
            | Event::MouseUp(mouse_event)
            | Event::MouseMove(mouse_event)
            | Event::MouseWheel(mouse_event) => Some(mouse_event.pos),
            Event::KeyDown(_) => None,
        }
    }

    pub fn offset(mut self, offset: (f64, f64)) -> Event {
        match &mut self {
            Event::MouseDown(mouse_event)
            | Event::MouseUp(mouse_event)
            | Event::MouseMove(mouse_event)
            | Event::MouseWheel(mouse_event) => {
                mouse_event.pos -= offset;
            }
            Event::KeyDown(_) => {}
        }
        self
    }

    pub fn listener(&self) -> Option<EventListner> {
        match self {
            Event::MouseDown(_) => None,
            Event::MouseUp(_) => None,
            Event::MouseMove(_) => None,
            Event::MouseWheel(_) => None,
            Event::KeyDown(_) => Some(EventListner::KeyDown),
        }
    }
}
