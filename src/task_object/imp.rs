// Object holding the state
#[derive(Default)]
pub struct TaskObject {
    pub data: Rc<RefCell<TaskData>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
    type Type = super::TaskObject;
}
