use super::*;

pub fn run() {
    box_t::demo();
    rc_t::demo();
    arc_t::demo();
    refcell_t::demo();
    mutex_t::demo();
    arc_mutex::demo();
    cell_t::demo();
    pin_t::demo();
    cow_t::demo();
}
