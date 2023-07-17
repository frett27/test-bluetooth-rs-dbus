extern crate dbus;

use dbus::blocking::Connection;
use std::time::Duration;

mod bluez;

fn introspect(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Second, create a wrapper struct around the connection that makes it easy
    // to send method calls to a specific destination and path.
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    // Now make the method call. The ListNames method call takes zero input parameters and
    // one output parameter which is an array of strings.
    // Therefore the input is a zero tuple "()", and the output is a single tuple "(names,)".
    let (names,): (Vec<String>,) = proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;

    // Let's print all the names to stdout.
    for name in names {
        println!("{}", name);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open up a connection to the session bus.
    let conn = Connection::new_system()?;

    introspect(&conn)?;

    use bluez::*;

    let p = conn.with_proxy("org.bluez", "/", Duration::from_secs(1));
    let r = p.get_managed_objects().unwrap();
    println!("result : {:?}", r);

    let r2 = p.introspect().unwrap();
    println!("result : {:?}", r2);

    Ok(())
}
