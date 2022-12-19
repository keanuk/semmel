glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

