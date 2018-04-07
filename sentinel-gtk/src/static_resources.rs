use gio::{resources_register, Error, Resource};
use glib::Bytes;
use gtk;

pub fn init() -> Result<(), Error> {
    let res_bytes = include_bytes!("../resources/resources.gresource");

    let gbytes = Bytes::from_static(res_bytes.as_ref());
    let resource = Resource::new_from_data(&gbytes)?;

    resources_register(&resource);
    Ok(())
}
