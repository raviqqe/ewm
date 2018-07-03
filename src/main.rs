use wlc::*;

struct Compositor;

impl Callback for Compositor {
    fn view_created(&mut self, v: &View) -> bool {
        v.set_visibility(v.output().visibility());
        v.bring_to_front();
        v.focus();
        true
    }

    fn view_focus(&mut self, v: &View, b: bool) {
        v.set_state(ViewState::Activated, b);
    }
}

fn main() {
    let _wlc = wlc::init(Compositor).unwrap();
}
