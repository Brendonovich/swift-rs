use build_utils::{link_swift, link_swift_package};

fn main() {
    link_swift();
    link_swift_package("SwiftRs", "./");
}
